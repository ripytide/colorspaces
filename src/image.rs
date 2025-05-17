use crate::*;

/// in row-column format
pub type Pixels<P> = Vec<Vec<P>>;

/// An Image
pub struct Image<P> {
    pixels: Pixels<P>,
    metadata: ImageMetadata,
}

impl<P> Image<P> {
    pub fn width(&self) -> usize {
        self.pixels.len()
    }

    pub fn height(&self) -> usize {
        if self.pixels.is_empty() {
            0
        } else {
            self.pixels[0].len()
        }
    }
}

/// Image metadata.
pub struct ImageMetadata {
    color_space: Option<ColorSpace>,
    alpha_type: Option<AlphaType>,
    original_file_size: Option<u32>,
    original_image_dimensions: Option<(u32, u32)>,
    file_modification_timestamp: Option<u64>,
    decoder_info: Option<DecoderInfo>,
}

/// The type of alpha compositing.
///
/// <https://en.wikipedia.org/wiki/Alpha_compositing>
pub enum AlphaType {
    Straight,
    Premultiplied,
}

/// Decoder information.
pub struct DecoderInfo {
    file_format: ImageFileFormat,
    decoder_name: String,
    decoder_version: String,
}

/// Image file formats.
///
/// https://en.wikipedia.org/wiki/Image_file_format
pub enum ImageFileFormat {
    Png,
    Jpeg,
}
