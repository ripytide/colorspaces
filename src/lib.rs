//! A crate for converting pixels and images between pixels types and
//! their respective color-spaces.
#![allow(dead_code)]
#![allow(unused_variables)]

mod colorspace;
mod decorerinfo;

pub use colorspace::ColorSpace;

pub struct Image<P> {
    pixels: Vec<P>,
    metadata: Metadata,
}

pub struct Metadata {
    colorspace: Option<ColorSpace>,
    original_file_size: u32,
    decoder_info: DecoderInfo,
}
