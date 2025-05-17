#![allow(dead_code)]
#![allow(unused_variables)]

mod preset;
pub use preset::SRGB;

mod colorspace;
pub use colorspace::ColorSpace;
pub use colorspace::{
    CmyColorSpace, CmykColorSpace, HslColorSpace, HsluvColorSpace, HsvColorSpace, LabColorSpace,
    LuvColorSpace, OklabColorSpace, RgbColorSpace, SyccColorSpace, UvwColorSpace, XvyccColorSpace,
    XyzColorSpace, YcbcrColorSpace, YdbdrColorSpace, YiqColorSpace, YpbprColorSpace, YuvColorSpace,
};
pub use colorspace::{Differencing, Luminance, Primaries, Transfer, Whitepoint};

mod image;
pub use image::{AlphaType, DecoderInfo, Image, ImageFileFormat, ImageMetadata, Pixels};
