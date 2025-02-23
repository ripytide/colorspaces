// types derived from https://docs.rs/image-canvas

use rgb::RgbColorSpace;
use yuv::YuvColorSpace;

pub mod transfer;
pub mod yuv;
pub mod rgb;

/// Identifies a color representation.
///
/// This names the model by which the numbers in the channels relate to a physical model. How
/// exactly depends on the variant as presented below. Some of them can be customized further with
/// parameters.
///
/// Notably, there are _NOT_ the numbers which we will use in image operations. Generally, we will
/// use an associated _linear_ representation of those colors instead. The choice here depends on
/// the color and is documented for each variants. It is chosen to provide models for faithful
/// linear operations on these colors such as mixing etc.
///
/// TODO: colors describe _paths_ to linear display, so we should somehow implement direction
/// conversions such as "BT.2087 : Colour conversion from Recommendation ITU-R BT.709 to
/// Recommendation ITU-R BT.2020" in a separate manner.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorSpace {
    Rgb(RgbColorSpace),
    Yuv(YuvColorSpace),
    /// The simple but perceptual space Oklab by Björn Ottoson.
    ///
    /// The _linear_ representation of this color is Lab but its quantized components are may be
    /// either Lab or LCh.
    ///
    /// It's based on a combination of two linear transforms and one non-linear power-function
    /// between them. Coefficients of these transforms are based on optimization against matching
    /// pairs in the detailed CAM16 model, trying to predict the parameters in those pairs as
    /// precisely as possible. For details see [the post's derivation][derivation].
    ///
    /// Reference: <https://bottosson.github.io/posts/oklab/>
    ///
    /// [derivation]: https://bottosson.github.io/posts/oklab/#how-oklab-was-derived
    Oklab,
    /// A group of scalar values, with no assigned relation to physical quantities.
    ///
    /// The purpose of this color is to simplify the process of creating color ramps and sampling
    /// functions, which do not have any interpretation themselves but are just coefficients to be
    /// used somewhere else.
    ///
    /// The only `SampleParts` that are allowed to be paired with this are `XYZ`.
    ///
    /// Additionally, you might use the images created with this color as an input or an
    /// intermediate step of a `transmute` to create images with chosen values in the linear
    /// representation without the need to manually calculate their texel encoding.
    Scalars {
        /// The transfer to use for points, as if they are RGB-ish colors.
        /// You can simply use `Linear` if you do not want to encode and rgb texel.
        transfer: Transfer,
    },
    /// A LAB space based on contemporary perceptual understanding.
    ///
    /// > The newly defined SRLAB2 color model is a compromise between the simplicity of CIELAB and
    /// > the correctness of CIECAM02.
    ///
    /// By combining whitepoint adaption in the (more) precise model of CIECAM02 while performing
    /// the transfer function in the cone response space, this achieves a good uniformity by
    /// simply modelling the human perception properly. It just leaves out the surround luminance
    /// model in the vastly more complex CIECAM02.
    ///
    /// This is lacking for HDR. This is because its based on L*ab which is inherently optimized
    /// for the small gamut of SDR. It's not constant luminance at exceedingly bright colors where
    /// ICtCp might provide a better estimate (compare ΔEITP, ITU-R Rec. BT.2124).
    ///
    /// Reference: <https://www.magnetkern.de/srlab2.html>
    SrLab2 {
        whitepoint: Whitepoint,
    },
}

/// The reference brightness of the color specification.
///
/// FIXME(color): scaling to reference luminance doesn't have an interface yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Luminance {
    /// 100cd/m².
    Sdr,
    /// 10_000cd/m².
    /// Known as high-dynamic range.
    Hdr,
    /// 160cd/m².
    AdobeRgb,
    /// 1000 nits, optimized for projector use.
    DciP3,
}

/// The relative stimuli of the three corners of a triangular RGBish gamut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The differencing scheme used in a Yuv construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The whitepoint/standard illuminant.
///
/// | Illuminant | X       | Y       | Z       |
/// |------------|---------|---------|---------|
/// | A          | 1.09850 | 1.00000 | 0.35585 |
/// | B          | 0.99072 | 1.00000 | 0.85223 |
/// | C          | 0.98074 | 1.00000 | 1.18232 |
/// | D50        | 0.96422 | 1.00000 | 0.82521 |
/// | D55        | 0.95682 | 1.00000 | 0.92149 |
/// | D65        | 0.95047 | 1.00000 | 1.08883 |
/// | D75        | 0.94972 | 1.00000 | 1.22638 |
/// | E          | 1.00000 | 1.00000 | 1.00000 |
/// | F2         | 0.99186 | 1.00000 | 0.67393 |
/// | F7         | 0.95041 | 1.00000 | 1.08747 |
/// | F11        | 1.00962 | 1.00000 | 0.64350 |
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// Transfer functions from encoded chromatic samples to physical quantity.
///
/// Ignoring viewing environmental effects, this describes a pair of functions that are each others
/// inverse: An electro-optical transfer (EOTF) and opto-electronic transfer function (OETF) that
/// describes how scene lighting is encoded as an electric signal. These are applied to each
/// stimulus value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
