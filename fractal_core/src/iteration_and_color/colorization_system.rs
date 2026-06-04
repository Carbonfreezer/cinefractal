//! This module contains everything related to the optional colorization of the system.

use crate::iteration_and_color::iteration_field_interface::ColorSystem;
use colorous::{COOL, Gradient, INFERNO, MAGMA, PLASMA, RAINBOW, TURBO, VIRIDIS};
use rayon::prelude::{ParallelIterator, IndexedParallelIterator};

/// Returns the normalized field (ranges from 0..1) for colorization purposes.
/// It calculates variations of the logarithm to scale the color.
/// It does so starting from a discrete iteration field.
fn normalize_discrete(
    input_iterator: impl IndexedParallelIterator<Item = u16>,
    max_iter: u16,
    log_strength: f32,
) -> impl IndexedParallelIterator<Item = f32> {
    let norm_factor = if log_strength <= 0.0 {
        1.0 / (max_iter as f32)
    } else {
        1.0 / (log_strength * max_iter as f32).ln_1p()
    };
    input_iterator.map(move |x| {
        let nominator = if log_strength <= 0.0 {
            x as f32
        } else {
            (log_strength * x as f32).ln_1p()
        };
        nominator * norm_factor
    })
}

/// Returns the normalized field (ranges from 0..1) for colorization purposes.
/// it calculates variations of the logarithm to scale the color.
/// It does so starting from a continuous iteration field.
fn normalize_continuous(
    input_iterator: impl IndexedParallelIterator<Item = f32>,
    max_iter: u16,
    log_strength: f32,
) -> impl IndexedParallelIterator<Item = f32> {
    let norm_factor = if log_strength <= 0.0 {
        1.0 / (max_iter as f32)
    } else {
        1.0 / (log_strength * max_iter as f32).ln_1p()
    };
    input_iterator.map(move |x| {
        let nominator = if log_strength <= 0.0 {
            x.max(0.0)
        } else {
            (log_strength * x.max(0.0)).ln_1p()
        };
        nominator * norm_factor
    })
}

impl ColorSystem {
    /// Gets the colours internal representation from our internal color system.
    fn get_colours_color(&self) -> Gradient {
        match &self {
            ColorSystem::Turbo => TURBO,
            ColorSystem::Viridis => VIRIDIS,
            ColorSystem::Magma => MAGMA,
            ColorSystem::Plasma => PLASMA,
            ColorSystem::Inferno => INFERNO,
            ColorSystem::Cool => COOL,
            ColorSystem::Cyclical { .. } => RAINBOW,
        }
    }
}

/// Creates an iterator for colors starting from a normalized field.
fn generate_color_iterator(
    color: ColorSystem,
    norm: impl IndexedParallelIterator<Item = f32>,
) -> impl ParallelIterator<Item = u8> {
    let internal_color = color.get_colours_color();
    let (is_cyclical, scaling) = match color {
        ColorSystem::Cyclical { repetition_factor } => (true, repetition_factor),
        _ => (false, 1.0),
    };
    norm.flat_map_iter(move |x| {
        let t = x * scaling;
        let t = if is_cyclical {
            t.fract()
        } else {
            t.clamp(0.0, 1.0)
        };
        let color = internal_color.eval_continuous(t as f64);
        [color.r, color.g, color.b]
    })
}

/// Crates a color field given a discrete iteration field.
pub fn colorize_discrete(
    input_iterator: impl IndexedParallelIterator<Item = u16>,
    max_iter: u16,
    log_strength: f32,
    color: ColorSystem,
) -> impl ParallelIterator<Item = u8> {
    let norm = normalize_discrete(input_iterator, max_iter, log_strength);
    generate_color_iterator(color, norm)
}

/// Creates a color field given a continuous iteration field.
pub fn colorize_continuous(
    input_iterator: impl IndexedParallelIterator<Item = f32>,
    max_iter: u16,
    log_strength: f32,
    color: ColorSystem,
) -> impl ParallelIterator<Item = u8> {
    let norm = normalize_continuous(input_iterator, max_iter, log_strength);
    generate_color_iterator(color, norm)
}
