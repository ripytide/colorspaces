//! Colorspace Types

/// A color space.
///
/// <https://en.wikipedia.org/wiki/Color_space>
/// <https://en.wikipedia.org/wiki/List_of_color_spaces_and_their_uses>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ColorSpace {
    /// CIE 1931 XYZ color space.
    ///
    /// <https://en.wikipedia.org/wiki/CIE_1931_color_space>
    Xyz(XyzColorSpace),
    /// CIE 1964 UVW color space.
    ///
    /// <https://en.wikipedia.org/wiki/CIE_1964_color_space>
    Uvw(UvwColorSpace),
    /// CIE 1976 LUV color space.
    ///
    /// <https://en.wikipedia.org/wiki/CIELUV>
    Luv(LuvColorSpace),
    /// CIE 1976 LAB color space.
    ///
    /// <https://en.wikipedia.org/wiki/CIELAB_color_space>
    Lab(LabColorSpace),
    /// HSLuv color space.
    ///
    /// <https://en.wikipedia.org/wiki/HSLuv>
    Hsluv(HsluvColorSpace),
    /// Oklab color space.
    ///
    /// <https://en.wikipedia.org/wiki/Oklab_color_space>
    Oklab(OklabColorSpace),
    /// Red Green Blue color space.
    ///
    /// <https://en.wikipedia.org/wiki/RGB_color_spaces>
    Rgb(RgbColorSpace),
    /// YUV color space.
    ///
    /// <https://en.wikipedia.org/wiki/Y%E2%80%B2UV>
    Yuv(YuvColorSpace),
    /// CMYK color space.
    ///
    /// <https://en.wikipedia.org/wiki/CMYK_color_model>
    Cmyk(CmykColorSpace),
    /// CMY color space.
    ///
    /// <https://simple.wikipedia.org/wiki/CMY_color_model>
    Cmy(CmyColorSpace),
    /// YCbCr color space.
    ///
    /// <https://en.wikipedia.org/wiki/YCbCr>
    Ycbcr(YcbcrColorSpace),
    /// YPbPr color space.
    ///
    /// <https://en.wikipedia.org/wiki/YPbPr>
    Ypbpr(YpbprColorSpace),
    /// YDbDr color space.
    ///
    /// <https://en.wikipedia.org/wiki/YDbDr>
    Ydbdr(YdbdrColorSpace),
    /// YIQ color space.
    ///
    /// <https://en.wikipedia.org/wiki/YIQ>
    Yiq(YiqColorSpace),
    /// xvYCC color space.
    ///
    /// <https://en.wikipedia.org/wiki/XvYCC>
    Xvycc(XvyccColorSpace),
    /// sYCC color space.
    ///
    /// <https://en.wikipedia.org/wiki/sYCC>
    Sycc(SyccColorSpace),
    /// Hue Saturation Value color space.
    ///
    /// <https://en.wikipedia.org/wiki/HSL_and_HSV>
    Hsv(HsvColorSpace),
    /// Hue Saturation Lightness color space.
    ///
    /// <https://en.wikipedia.org/wiki/HSL_and_HSV>
    Hsl(HslColorSpace),
}

/// A reference luminance for a color space.
///
/// <https://en.wikipedia.org/wiki/Luminance>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Luminance {
    /// Standard-Dynamic-Range.
    ///
    /// Maximum luminance: 100 cd/m²
    ///
    /// <https://en.wikipedia.org/wiki/Standard-dynamic-range_video>
    Sdr,
    /// High-Dynamic-Range.
    ///
    /// Maximum luminance: 10_000 cd/m²
    ///
    /// <https://en.wikipedia.org/wiki/High_dynamic_range>
    Hdr,
    /// Adobe RGB reference luminance.
    ///
    /// Maximum luminance: 160 cd/m²
    ///
    /// <https://en.wikipedia.org/wiki/Adobe_RGB_color_space>
    AdobeRgb,
    /// DCI-P3 reference luminance.
    ///
    /// Maximum luminance: 1000 cd/m²
    ///
    /// <https://en.wikipedia.org/wiki/DCI-P3>
    DciP3,
}

/// A relative stimuli of the three corners of a triangular RGBish gamut.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Primaries {
    /// The CIE XYZ 'primaries'.
    /// FIXME(color): does this really make sense?
    Xyz,
    /// First set of primaries specified in Bt/Rec.601.
    ///
    /// These are actually the same as in SMPTE240M.
    Bt601_525,
    /// Second set of primaries specified in Bt/Rec.601.
    Bt601_625,
    /// Primaries specified in Bt/Rec.709.
    Bt709,
    /// Primaries specified in SMPTE240-M.
    ///
    /// There are actually the same as BT.601.
    Smpte240,
    /// Primaries specified in Bt/Rec.2020.
    ///
    /// Also known as Wide Color Gamut.
    Bt2020,
    /// Primaries specified in Bt/Rec.2100.
    ///
    /// Also known as Wide Color Gamut. See Bt.2020.
    Bt2100,
}

/// A differencing scheme used in [`YuvColorSpace`].
///
/// <https://en.wikipedia.org/wiki/Y%E2%80%B2UV>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Differencing {
    /// Rec BT.470 M/PAL differencing scheme for E_U and E_V, the naming origin for 'YUV'.
    /// FIXME: add YIQ proper, to add BT.470 M/NTSC?
    ///
    /// Note this same differencing scheme is used with different color primaries and whitepoints.
    /// With those shared with BT601_625 and D65 in more modern systems and a different one under
    /// illuminant C.
    Bt407MPal,
    /// The BT.470 M/PAL has a typo and, based on its parameters, we can derive a more accurate
    /// version than as what was published..
    Bt407MPalPrecise,
    /// Rec BT.601 luminance differencing.
    Bt601,
    /// Rec BT.601 luminance differencing, quantized with headroom.
    /// This is intended for analog use, not for digital images.
    Bt601Quantized,
    /// Rec BT.601 luminance differencing, quantized without headroom.
    ///
    /// Please tell the crate author where it's used but this makes it easy to quantize to 8-bit
    /// unsigned integers.
    Bt601FullSwing,
    /// Rec BT.709 luminance differencing.
    Bt709,
    /// Analog form
    Bt709Quantized,
    /// Rec BT.709 luminance differencing, quantized without headroom.
    /// Not technically an ITU BT recommendation, but introduced in h.264.
    Bt709FullSwing,

    // TODO: Rec. ITU-R BT.1361 = BT709 with a dash of questionable 'extended gamut quantization'.
    // Suppressed at (suppressed on 12/02/15) in favor of BT2020 (published xx/10/15).
    // But then again, it's referenced by EBU: https://tech.ebu.ch/docs/tech/tech3299.pdf
    // Turtles all the way down.
    /// Factors from analog SECAM standard.
    YDbDr,
    /// Rec BT.2020 luminance differencing.
    Bt2020,
    /// Rec BT.2100 luminance differencing.
    /// Same coefficients as the BT2020 scheme.
    Bt2100,
    /// Differencing scheme from YCoCb/ITU-T H.273.
    YCoCg,
}

/// A white point.
///
/// <https://en.wikipedia.org/wiki/White_point>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Whitepoint {
    A,
    B,
    C,
    D50,
    D55,
    D65,
    D75,
    E,
    F2,
    F7,
    F11,
}

/// A transfer function.
///
/// <https://en.wikipedia.org/wiki/Transfer_functions_in_imaging>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[non_exhaustive]
pub enum Transfer {
    /// Non-linear electrical data of Bt.709
    Bt709,
    Bt470M,
    /// Non-linear electrical data of Bt.601
    Bt601,
    /// Non-linear electrical data of Smpte-240
    Smpte240,
    /// Linear color in display luminance.
    Linear,
    /// Non-linear electrical data of Srgb
    ///
    /// Technically, we're implementing scRGB since we handle negative primaries just well enough.
    Srgb,
    /// Non-linear electrical data of Bt2020 that was 10-bit quantized
    Bt2020_10bit,
    /// Non-linear electrical data of Bt2020 that was 12-bit quantized
    /// FIXME(color): not yet supported, panics on use.
    Bt2020_12bit,
    /// Non-linear electrical data of Smpte-2048
    Smpte2084,
    /// Another name for Smpte2084.
    /// FIXME(color): not yet supported, panics on use.
    Bt2100Pq,
    /// Non-linear electrical data of Bt2100 Hybrid-Log-Gamma.
    /// FIXME(color): not yet supported, panics on use.
    Bt2100Hlg,
    /// Linear color in scene luminance of Bt2100.
    /// This is perfect for an artistic composition pipeline. The rest of the type system will
    /// ensure this is not accidentally and unwittingly mixed with `Linear` but otherwise this is
    /// treated as `Linear`. You might always transmute.
    /// FIXME(color): not yet supported, panics on use.
    Bt2100Scene,
}

/// CIE 1931 XYZ color space.
///
/// <https://en.wikipedia.org/wiki/CIE_1931_color_space>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XyzColorSpace {}

/// CIE 1964 UVW color space.
///
/// <https://en.wikipedia.org/wiki/CIE_1964_color_space>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UvwColorSpace {}

/// CIE 1976 LUV color space.
///
/// <https://en.wikipedia.org/wiki/CIELUV>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LuvColorSpace {}

/// CIE 1976 LAB color space.
///
/// <https://en.wikipedia.org/wiki/CIELAB_color_space>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabColorSpace {}

/// HSLuv color space.
///
/// <https://en.wikipedia.org/wiki/HSLuv>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HsluvColorSpace {}

/// Oklab color space.
///
/// <https://en.wikipedia.org/wiki/Oklab_color_space>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OklabColorSpace {}

/// Red Green Blue color space.
///
/// <https://en.wikipedia.org/wiki/RGB_color_spaces>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RgbColorSpace {
    pub primaries: Primaries,
    pub transfer: Transfer,
    pub whitepoint: Whitepoint,
    pub luminance: Luminance,
}

/// YUV color space.
///
/// <https://en.wikipedia.org/wiki/Y%E2%80%B2UV>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct YuvColorSpace {
    pub primary: Primaries,
    pub whitepoint: Whitepoint,
    pub transfer: Transfer,
    pub luminance: Luminance,
    pub differencing: Differencing,
}

/// CMYK color space.
///
/// <https://en.wikipedia.org/wiki/CMYK_color_model>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CmykColorSpace {}

/// CMY color space.
///
/// <https://simple.wikipedia.org/wiki/CMY_color_model>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CmyColorSpace {}

/// YCbCr color space.
///
/// <https://en.wikipedia.org/wiki/YCbCr>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct YcbcrColorSpace {}

/// YPbPr color space.
///
/// <https://en.wikipedia.org/wiki/YPbPr>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct YpbprColorSpace {}

/// YDbDr color space.
///
/// <https://en.wikipedia.org/wiki/YDbDr>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct YdbdrColorSpace {}

/// YIQ color space.
///
/// <https://en.wikipedia.org/wiki/YIQ>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct YiqColorSpace {}

/// xvYCC color space.
///
/// <https://en.wikipedia.org/wiki/XvYCC>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XvyccColorSpace {}

/// sYCC color space.
///
/// <https://en.wikipedia.org/wiki/sYCC>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyccColorSpace {}

/// Hue Saturation Value color space.
///
/// <https://en.wikipedia.org/wiki/HSL_and_HSV>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HsvColorSpace {}

/// Hue Saturation Lightness color space.
///
/// <https://en.wikipedia.org/wiki/HSL_and_HSV>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HslColorSpace {}
