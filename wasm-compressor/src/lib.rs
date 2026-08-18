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

/// Keeps the original public API for PNG callers.
#[wasm_bindgen]
pub fn compress_png(input_bytes: &[u8], quality: u8) -> Result<Vec<u8>, JsValue> {
    compress_image(input_bytes, quality, "png")
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

    let (palette, indexed_pixels) = quantization
        .remapped(&mut quantized_image)
        .map_err(|error| JsValue::from_str(&format!("failed to remap image: {error}")))?;

    let bytes = match format.to_ascii_lowercase().as_str() {
        "png" => encode_png(width, height, &palette, &indexed_pixels),
        "jpg" | "jpeg" => {
            let pixels = expand_palette(&palette, &indexed_pixels);
            let mut output = Cursor::new(Vec::new());
            let mut encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
            encoder
                .encode(&pixels, width, height, image::ExtendedColorType::Rgba8)
                .map_err(|error| JsValue::from_str(&format!("failed to encode JPEG: {error}")))?;
            Ok(output.into_inner())
        }
        "webp" => {
            let pixels = expand_palette(&palette, &indexed_pixels);
            let mut output = Cursor::new(Vec::new());
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut output);
            encoder
                .encode(&pixels, width, height, image::ExtendedColorType::Rgba8)
                .map_err(|error| JsValue::from_str(&format!("failed to encode WebP: {error}")))?;
            Ok(output.into_inner())
        }
        _ => Err(JsValue::from_str("unsupported output format")),
    }?;

    Ok(CompressionResult {
        bytes,
        compression_ms: js_sys::Date::now() - started_at,
    })
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

fn expand_palette(palette: &[imagequant::RGBA], pixels: &[u8]) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(|index| {
            let color = palette[*index as usize];
            [color.r, color.g, color.b, color.a]
        })
        .collect()
}
