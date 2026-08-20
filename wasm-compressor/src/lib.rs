use std::io::Cursor;

use exif::{In, Tag};
use image::metadata::Orientation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CompressionResult {
    bytes: Vec<u8>,
    compression_ms: f64,
}

#[wasm_bindgen]
impl CompressionResult {
    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn compression_ms(&self) -> f64 {
        self.compression_ms
    }
}

/// Decode, quantize, and encode an image as png, jpeg, or webp.
#[wasm_bindgen]
pub fn compress_image(input_bytes: &[u8], quality: u8, format: &str) -> Result<Vec<u8>, JsValue> {
    Ok(compress_image_with_metadata(input_bytes, quality, format)?.bytes)
}

/// Compress an image and return the encoded bytes with Rust-side timing metadata.
#[wasm_bindgen]
pub fn compress_image_with_metadata(
    input_bytes: &[u8],
    quality: u8,
    format: &str,
) -> Result<CompressionResult, JsValue> {
    let started_at = js_sys::Date::now();

    let mut image = image::load_from_memory(input_bytes)
        .map_err(|error| JsValue::from_str(&format!("failed to decode image: {error}")))?;
    if let Some(orientation) = read_exif_orientation(input_bytes) {
        image.apply_orientation(orientation);
    }
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();

    // PNG and WebP use palette quantization to make quality affect their output.
    // JPEG is a full-color format, so it encodes the raw RGB pixels directly.
    let bytes = match format.to_ascii_lowercase().as_str() {
        "png" => {
            let (palette, indexed_pixels) = quantize_rgba(&rgba, quality)?;
            encode_png(width, height, &palette, &indexed_pixels)
        }
        "jpg" | "jpeg" => {
            let rgb = flatten_rgba_to_rgb(rgba.as_raw(), [255, 255, 255]);
            encode_jpeg(&rgb, width, height, quality)
        }
        "webp" => {
            let (palette, indexed_pixels) = quantize_webp_rgba(&rgba, quality)?;
            let quantized_rgba = indexed_pixels
                .into_iter()
                .flat_map(|index| {
                    let color = palette[index as usize];
                    [color.r, color.g, color.b, color.a]
                })
                .collect::<Vec<_>>();
            encode_webp(&quantized_rgba, width, height)
        }
        _ => Err(JsValue::from_str("unsupported output format")),
    }?;

    Ok(CompressionResult {
        bytes,
        compression_ms: js_sys::Date::now() - started_at,
    })
}

fn read_exif_orientation(bytes: &[u8]) -> Option<Orientation> {
    let mut reader = Cursor::new(bytes);
    let exif_reader = exif::Reader::new();
    let exif_data = exif_reader.read_from_container(&mut reader).ok()?;
    let field = exif_data.get_field(Tag::Orientation, In::PRIMARY)?;
    field
        .value
        .get_uint(0)
        .and_then(|value| u8::try_from(value).ok())
        .and_then(Orientation::from_exif)
}

fn quantize_rgba(
    rgba: &image::RgbaImage,
    quality: u8,
) -> Result<(Vec<imagequant::RGBA>, Vec<u8>), JsValue> {
    quantize_rgba_with_max_colors(rgba, quality, 256)
}

fn quantize_webp_rgba(
    rgba: &image::RgbaImage,
    quality: u8,
) -> Result<(Vec<imagequant::RGBA>, Vec<u8>), JsValue> {
    let max_colors = 16 + u32::from(quality) * 240 / 100;
    quantize_rgba_with_max_colors(rgba, quality, max_colors)
}

fn quantize_rgba_with_max_colors(
    rgba: &image::RgbaImage,
    quality: u8,
    max_colors: u32,
) -> Result<(Vec<imagequant::RGBA>, Vec<u8>), JsValue> {
    let (width, height) = rgba.dimensions();

    let mut attributes = imagequant::new();
    attributes
        .set_quality(0, quality)
        .map_err(|error| JsValue::from_str(&format!("failed to set quality: {error}")))?;
    attributes
        .set_max_colors(max_colors)
        .map_err(|error| JsValue::from_str(&format!("failed to configure palette: {error}")))?;

    let source_pixels = rgba
        .as_raw()
        .chunks_exact(4)
        .map(|pixel| imagequant::RGBA {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3],
        })
        .collect::<Vec<_>>();
    let mut quantized_image = attributes
        .new_image(source_pixels, width as usize, height as usize, 0.0)
        .map_err(|error| JsValue::from_str(&format!("failed to prepare image: {error}")))?;
    let mut quantization = attributes
        .quantize(&mut quantized_image)
        .map_err(|error| JsValue::from_str(&format!("failed to quantize image: {error}")))?;
    quantization
        .set_dithering_level(1.0)
        .map_err(|error| JsValue::from_str(&format!("failed to configure dithering: {error}")))?;

    quantization
        .remapped(&mut quantized_image)
        .map_err(|error| JsValue::from_str(&format!("failed to remap image: {error}")))
}

fn flatten_rgba_to_rgb(rgba: &[u8], background: [u8; 3]) -> Vec<u8> {
    let bg = [
        background[0] as f32,
        background[1] as f32,
        background[2] as f32,
    ];
    rgba.chunks_exact(4)
        .flat_map(|pixel| {
            let alpha = pixel[3] as f32 / 255.0;
            let inverse = 1.0 - alpha;
            [
                (pixel[0] as f32 * alpha + bg[0] * inverse).round() as u8,
                (pixel[1] as f32 * alpha + bg[1] * inverse).round() as u8,
                (pixel[2] as f32 * alpha + bg[2] * inverse).round() as u8,
            ]
        })
        .collect()
}

fn encode_jpeg(rgb: &[u8], width: u32, height: u32, quality: u8) -> Result<Vec<u8>, JsValue> {
    let mut output = Cursor::new(Vec::new());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
    encoder
        .encode(rgb, width, height, image::ExtendedColorType::Rgb8)
        .map_err(|error| JsValue::from_str(&format!("failed to encode JPEG: {error}")))?;
    Ok(output.into_inner())
}

fn encode_webp(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, JsValue> {
    let mut output = Cursor::new(Vec::new());
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut output);
    encoder
        .encode(rgba, width, height, image::ExtendedColorType::Rgba8)
        .map_err(|error| JsValue::from_str(&format!("failed to encode WebP: {error}")))?;
    Ok(output.into_inner())
}

fn encode_png(
    width: u32,
    height: u32,
    palette: &[imagequant::RGBA],
    pixels: &[u8],
) -> Result<Vec<u8>, JsValue> {
    let mut palette_bytes = Vec::with_capacity(palette.len() * 3);
    let mut transparency = Vec::with_capacity(palette.len());
    for color in palette {
        palette_bytes.extend_from_slice(&[color.r, color.g, color.b]);
        transparency.push(color.a);
    }

    let mut output = Cursor::new(Vec::new());
    {
        let mut encoder = png::Encoder::new(&mut output, width, height);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_palette(palette_bytes);
        encoder.set_trns(transparency);
        let mut writer = encoder
            .write_header()
            .map_err(|error| JsValue::from_str(&format!("failed to write PNG header: {error}")))?;
        writer
            .write_image_data(pixels)
            .map_err(|error| JsValue::from_str(&format!("failed to write PNG data: {error}")))?;
    }
    Ok(output.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    fn orientation_tiff(value: u16) -> Vec<u8> {
        vec![
            b'I',
            b'I',
            42,
            0,
            8,
            0,
            0,
            0, // TIFF header and IFD offset
            1,
            0, // one IFD entry
            0x12,
            0x01, // Orientation tag
            3,
            0, // SHORT
            1,
            0,
            0,
            0, // one value
            (value & 0xff) as u8,
            (value >> 8) as u8,
            0,
            0,
            0,
            0,
            0,
            0, // next IFD offset
        ]
    }

    #[test]
    fn reads_exif_orientation_from_tiff_short() {
        assert_eq!(
            read_exif_orientation(&orientation_tiff(6)),
            Some(Orientation::Rotate90)
        );
    }

    #[test]
    fn maps_all_exif_orientations() {
        let expected = [
            Orientation::NoTransforms,
            Orientation::FlipHorizontal,
            Orientation::Rotate180,
            Orientation::FlipVertical,
            Orientation::Rotate90FlipH,
            Orientation::Rotate90,
            Orientation::Rotate270FlipH,
            Orientation::Rotate270,
        ];

        for (value, orientation) in (1..=8).zip(expected) {
            assert_eq!(Orientation::from_exif(value), Some(orientation));
        }
    }

    #[test]
    fn applies_rotation_to_pixels_and_dimensions() {
        let mut image = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            2,
            3,
            image::Rgba([255, 0, 0, 255]),
        ));

        image.apply_orientation(Orientation::Rotate90);

        assert_eq!(image.dimensions(), (3, 2));
    }

    #[test]
    fn webp_quality_changes_encoded_output() {
        let width = 128;
        let height = 128;
        let rgba = (0..width * height)
            .flat_map(|index| {
                let x = index % width;
                let y = index / width;
                [
                    ((x * 17 + y * 13) % 256) as u8,
                    ((x * 7 + y * 29) % 256) as u8,
                    ((x * 37 + y * 3) % 256) as u8,
                    255,
                ]
            })
            .collect::<Vec<_>>();

        let low_quality = quantized_webp(&rgba, width, height, 10);
        let high_quality = quantized_webp(&rgba, width, height, 99);

        assert_ne!(low_quality, high_quality);
        assert_ne!(low_quality.len(), high_quality.len());
    }

    fn quantized_webp(rgba: &[u8], width: u32, height: u32, quality: u8) -> Vec<u8> {
        let image = image::RgbaImage::from_raw(width, height, rgba.to_vec()).unwrap();
        let (palette, indexed_pixels) = quantize_webp_rgba(&image, quality).unwrap();
        let quantized_rgba = indexed_pixels
            .into_iter()
            .flat_map(|index| {
                let color = palette[index as usize];
                [color.r, color.g, color.b, color.a]
            })
            .collect::<Vec<_>>();
        encode_webp(&quantized_rgba, width, height).unwrap()
    }
}
