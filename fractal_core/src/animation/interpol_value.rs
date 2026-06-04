//! This module contains all elements that are interpolatable.

/// A smoothstep function to have vanishing derivatives.
fn smooth_step(x: f32) -> f64 {
    (x * x * (3.0 - 2.0 * x)) as f64
}

/// Trait for values that can be interpolated by an `interpol_value` in `0..=1`,
/// where 0 yields `self` and 1 yields the `partner`.
pub trait Interpolatable {
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self;
}

/// Integers are interpolated linearly in floating point and then rounded to the
/// nearest value, so an animated integer parameter visibly steps through the
/// intermediate values over time (e.g. an exponent 2 -> 6 steps 2,3,4,5,6).
/// Unlike the floating-point types below, integers are *not* eased through
/// `smooth_step`; the endpoints are still reproduced exactly (linear
/// interpolation yields `self` at 0 and `partner` at 1).
impl Interpolatable for u16 {
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self {
        let from = *self as f64;
        let to = *partner as f64;
        (from + interpol_value as f64 * (to - from)).round() as u16
    }
}

impl Interpolatable for u32 {
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self {
        let from = *self as f64;
        let to = *partner as f64;
        (from + interpol_value as f64 * (to - from)).round() as u32
    }
}

impl Interpolatable for f32 {
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self {
        let smooth = smooth_step(interpol_value) as f32;
        self + smooth * (partner - self)
    }
}

/// This represents a complex number to the outside for the animation system.
#[derive(Clone, Copy)]
pub struct PseudoComplex {
    pub real: f64,
    pub imag: f64,
}

impl PseudoComplex {
    pub fn new(real: f64, imag: f64) -> PseudoComplex {
        PseudoComplex { real, imag }
    }
}

impl Interpolatable for PseudoComplex {
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self {
        let smooth = smooth_step(interpol_value);
        Self::new(
            self.real + smooth * (partner.real - self.real),
            self.imag + smooth * (partner.imag - self.imag),
        )
    }
}

/// This is a special scaling value, because here we interpolate in logarithmic space.
#[derive(Clone)]
pub struct SpatialExtension(f64);

impl From<f64> for SpatialExtension {
    fn from(value: f64) -> Self {
        SpatialExtension(value)
    }
}

impl From<SpatialExtension> for f64 {
    fn from(value: SpatialExtension) -> Self {
        value.0
    }
}

impl Interpolatable for SpatialExtension {
    /// Interpolates the value in logarithmic space.
    fn interpolate(&self, partner: &Self, interpol_value: f32) -> Self {
        let smooth = smooth_step(interpol_value);
        let a = self.0.log2();
        let b = partner.0.log2();
        let res = (a + smooth * (b - a)).exp2();
        res.into()
    }
}
