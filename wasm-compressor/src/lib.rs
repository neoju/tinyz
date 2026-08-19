use std::io::Cursor;

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
    let image = image::load_from_memory(input_bytes)
        .map_err(|error| JsValue::from_str(&format!("failed to decode image: {error}")))?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();

    // PNG is palette-based, so it quantizes to a limited palette. JPEG and WebP
    // are full-color formats: encoding them from a 256-color palette destroys
    // photographic tonal gradations, so they encode the raw RGBA directly.
    let bytes = match format.to_ascii_lowercase().as_str() {
        "png" => {
            let (palette, indexed_pixels) = quantize_rgba(&rgba, quality)?;
            encode_png(width, height, &palette, &indexed_pixels)
        }
        "jpg" | "jpeg" => {
            let rgb = flatten_rgba_to_rgb(rgba.as_raw(), [255, 255, 255]);
            encode_jpeg(&rgb, width, height, quality)
        }
        "webp" => encode_webp(rgba.as_raw(), width, height),
        _ => Err(JsValue::from_str("unsupported output format")),
    }?;

    Ok(CompressionResult {
        bytes,
        compression_ms: js_sys::Date::now() - started_at,
    })
}

fn quantize_rgba(
    rgba: &image::RgbaImage,
    quality: u8,
) -> Result<(Vec<imagequant::RGBA>, Vec<u8>), JsValue> {
    let (width, height) = rgba.dimensions();

    let mut attributes = imagequant::new();
    attributes
        .set_quality(0, quality)
        .map_err(|error| JsValue::from_str(&format!("failed to set quality: {error}")))?;
    attributes
        .set_max_colors(256)
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
    let bg = [background[0] as f32, background[1] as f32, background[2] as f32];
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

