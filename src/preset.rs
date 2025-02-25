//! Preset Colorspace constants for colorspaces with parameters such as [`RgbColorSpace`]

use crate::{Luminance, Primaries, RgbColorSpace, Transfer, Whitepoint};

pub const SRGB: RgbColorSpace = RgbColorSpace {
    primaries: Primaries::Bt709,
    transfer: Transfer::Srgb,
    whitepoint: Whitepoint::D65,
    luminance: Luminance::Sdr,
};
