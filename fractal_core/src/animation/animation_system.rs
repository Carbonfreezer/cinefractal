//! This module contains the complete animation system.

use crate::animation::interpol_value::{PseudoComplex, SpatialExtension};
use crate::animation::key_frame_system::{ClosedKeyFrameList, KeyFrameList};
use crate::iteration_and_color::iteration_field_interface::IterationFieldInterface;

/// The values the animation system is seeded with at time 0.
///
/// The `Default` implementation mirrors the defaults of
/// [`IterationFieldInterface`], so an unconfigured animation system renders the
/// same start frame as an unconfigured field interface.
#[derive(Clone, Copy)]
pub struct AnimationStartConfiguration {
    pub exponent: u32,
    pub seed_real: f64,
    pub seed_imag: f64,
    pub center_point_real: f64,
    pub center_point_imag: f64,
    pub extension: f64,
    pub max_iterations: u16,
    pub log_strength: f32,
}

impl Default for AnimationStartConfiguration {
    fn default() -> Self {
        Self {
            exponent: 2,
            seed_real: 0.0,
            seed_imag: 0.0,
            center_point_real: 0.0,
            center_point_imag: 0.0,
            extension: 1.5,
            max_iterations: 1000,
            log_strength: 0.0,
        }
    }
}

/// Mutable builder for an animation.
///
/// Key frames are added in this open state. Calling
/// [`AnimationRecorder::into_player`] freezes every track and returns an
/// [`AnimationPlayer`] that can be queried by shared reference (and thus
/// rendered in parallel).
#[derive(Clone)]
pub struct AnimationRecorder {
    exponent: KeyFrameList<u32>,
    fractal_seed_value: KeyFrameList<PseudoComplex>,
    render_center_point: KeyFrameList<PseudoComplex>,
    extension: KeyFrameList<SpatialExtension>,
    max_iterations: KeyFrameList<u16>,
    log_strength_for_color: KeyFrameList<f32>,
}

impl Default for AnimationRecorder {
    fn default() -> Self {
        Self::new(AnimationStartConfiguration::default())
    }
}

impl AnimationRecorder {
    /// The recorder gets constructed with the start values being set for time 0.
    pub fn new(config: AnimationStartConfiguration) -> Self {
        let exponent = KeyFrameList::new(config.exponent);
        let fractal_seed_value = KeyFrameList::new(PseudoComplex::new(
            config.seed_real,
            config.seed_imag,
        ));
        let render_center_point = KeyFrameList::new(PseudoComplex::new(
            config.center_point_real,
            config.center_point_imag,
        ));
        let extension = KeyFrameList::new(config.extension.into());
        let max_iterations = KeyFrameList::new(config.max_iterations);
        let log_strength_for_color = KeyFrameList::new(config.log_strength);
        Self {
            exponent,
            fractal_seed_value,
            render_center_point,
            extension,
            max_iterations,
            log_strength_for_color,
        }
    }
    /// Adds a key frame for the exponent in the iteration. The exponent must be
    /// at least 2; otherwise an `Err` is returned and no key frame is added.
    pub fn set_keyframe_exponent(&mut self, time: f32, exponent: u32) -> Result<(), &'static str> {
        if exponent < 2 {
            Err("exponent in key frame must be at least 2")
        } else {
            self.exponent.add_key_frame(time, exponent);
            Ok(())
        }
    }

    /// Adds a key frame for the julia start value.
    pub fn set_keyframe_fractal_seed_value(&mut self, time: f32, real: f64, imag: f64) {
        self.fractal_seed_value
            .add_key_frame(time, PseudoComplex::new(real, imag));
    }

    /// Adds a key frame for the center point where to  render to.
    pub fn set_keyframe_render_center_point(&mut self, time: f32, real: f64, imag: f64) {
        self.render_center_point
            .add_key_frame(time, PseudoComplex::new(real, imag));
    }

    /// Sets a keyframe for the extension (zooming).
    pub fn set_keyframe_extension(&mut self, time: f32, extension: f64) -> Result<(), &'static str> {
        // The extension is interpolated in log space (log2), which requires a
        // strictly positive, finite value. The negated form also rejects NaN
        // (every comparison with NaN is false).
        if !(extension > 0.0 && extension.is_finite()) {
            return Err("extension must be a finite value greater than 0");
        }
        self.extension.add_key_frame(time, extension.into());
        Ok(())
    }

    /// The keyframe for the maximum number of iterations.
    pub fn set_keyframe_max_iterations(&mut self, time: f32, max_iter: u16) {
        self.max_iterations.add_key_frame(time, max_iter);
    }

    /// Sets the key frame for the log strength.
    pub fn set_keyframe_log_strength(&mut self, time: f32, log_strength: f32) {
        self.log_strength_for_color
            .add_key_frame(time, log_strength);
    }

    /// Consumes the builder, sorts every track and returns the immutable,
    /// queryable animation.
    pub fn into_player(self) -> AnimationPlayer {
        let exponent = self.exponent.into_closed();
        let fractal_seed_value = self.fractal_seed_value.into_closed();
        let render_center_point = self.render_center_point.into_closed();
        let extension = self.extension.into_closed();
        let max_iterations = self.max_iterations.into_closed();
        let log_strength_for_color = self.log_strength_for_color.into_closed();

        // The latest key frame time across all tracks. Constant once closed,
        // so we compute it here instead of on every query.
        let max_time = [
            exponent.max_time(),
            fractal_seed_value.max_time(),
            render_center_point.max_time(),
            extension.max_time(),
            max_iterations.max_time(),
            log_strength_for_color.max_time(),
        ]
        .into_iter()
        .fold(0.0_f32, f32::max);

        AnimationPlayer {
            exponent,
            fractal_seed_value,
            render_center_point,
            extension,
            max_iterations,
            log_strength_for_color,
            max_time,
        }
    }
}

/// Immutable, queryable animation produced by [`AnimationRecorder::into_player`].
///
/// All access is through `&self`, so a player is `Sync` and a single
/// instance can be shared across threads to render frames in parallel.
pub struct AnimationPlayer {
    exponent: ClosedKeyFrameList<u32>,
    fractal_seed_value: ClosedKeyFrameList<PseudoComplex>,
    render_center_point: ClosedKeyFrameList<PseudoComplex>,
    extension: ClosedKeyFrameList<SpatialExtension>,
    max_iterations: ClosedKeyFrameList<u16>,
    log_strength_for_color: ClosedKeyFrameList<f32>,
    max_time: f32,
}

impl AnimationPlayer {
    /// The end time of the animation: the latest key frame time across all
    /// tracks. Computed once in [`AnimationRecorder::into_player`]. Useful as the total
    /// duration when rendering or looping.
    pub fn max_time(&self) -> f32 {
        self.max_time
    }

    /// Applies all animated values at the given time to the field interface.
    pub fn apply_animation(&self, time: f32, interface: &mut IterationFieldInterface) {
        interface.set_fractal_exponent(self.exponent.get_interpolated_value(time));
        let seed = self.fractal_seed_value.get_interpolated_value(time);
        interface.set_fractal_seed_value(seed.real, seed.imag);
        let center = self.render_center_point.get_interpolated_value(time);
        interface.set_center_point(center.real, center.imag);
        interface.set_extension(self.extension.get_interpolated_value(time).into());
        interface.set_maximum_iterations(self.max_iterations.get_interpolated_value(time));
        interface.set_colorization_log_strength(
            self.log_strength_for_color.get_interpolated_value(time),
        );
    }
}
