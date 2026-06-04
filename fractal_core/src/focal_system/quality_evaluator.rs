//! This module contains the functionality to evaluate the quality of a sample point.

use crate::iteration_and_color::complex_number::ComplexNumber;
use crate::focal_system::focal_point::{FocalPoint, FocalPointCollection};
use crate::iteration_and_color::fractal_collection::AnyEscapeIteration;
use crate::iteration_and_color::iteration_field_interface::FractalConfig;
use crate::iteration_and_color::iteration_system::find_iteration_on_complex_number;
use itertools::iproduct;
use rand::RngExt;
use rand::prelude::ThreadRng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::f64::consts::TAU;

/// The scaling factor we apply to the incoming evaluation extension and the step in the loop.
/// The analysis window should cover 7.5% of the analysis range.
const DELTA_FACTOR: f64 = 0.075 / (2 * HALF_EXTENSION + 1) as f64;

/// Contains the half extension of  the variance calculation window.
const HALF_EXTENSION: i32 = 7;

/// The amount of samples we get for the variance calculation
const SAMPLES_FOR_VARIANCE: f64 = (2 * HALF_EXTENSION + 1).pow(2) as f64;

/// The factor we apply to the number of samples we want to get the amount of samples we want to probe.
const SAMPLE_AMOUNT_FACTOR: usize = 500;


/// Evaluates the level of interest of a probing point based on the variance of
/// the iteration counts in a small window around it.
/// `evaluation_extension` is the rendering extension at which this is measured
/// (a smaller value means a deeper zoom) and is fully decoupled from the search radius.
fn get_potential_focal_point(
    point: ComplexNumber,
    evaluation_extension: f64,
    iterator: AnyEscapeIteration,
    max_iterations: u16,
) -> FocalPoint {
    let step = DELTA_FACTOR * evaluation_extension;

    let (sum, sq_sum) = iproduct!(-HALF_EXTENSION..=HALF_EXTENSION, -HALF_EXTENSION..=HALF_EXTENSION)
        .fold((0.0, 0.0), |(sum, sq_sum), (dx, dy)| {
            let sample_position = ComplexNumber::new(
                dx as f64 * step,
                dy as f64 * step,
            ) + point;
            let (iter, _) = find_iteration_on_complex_number(iterator, sample_position, max_iterations);
            let value = iter as f64 / max_iterations as f64;
            (sum + value, sq_sum + value * value)
        });

    let mean = sum / SAMPLES_FOR_VARIANCE;
    let evaluation = (sq_sum / SAMPLES_FOR_VARIANCE - mean * mean).max(0.0);

    FocalPoint { point, evaluation }
}

/// Gets a complex number in min and max range radius area of a circle.
/// Makes sure that the random numbers are evenly distributed in the circle.
fn get_circle_random_complex_number(
    rng: &mut ThreadRng,
    min_range: f64,
    max_range: f64,
) -> ComplexNumber {
    let angle: f64 = rng.random::<f64>() * TAU;
    let radius = (min_range * min_range
        + rng.random::<f64>() * (max_range * max_range - min_range * min_range))
        .sqrt();

    ComplexNumber::new(radius * angle.cos(), radius * angle.sin())
}

/// Where to look: an annulus around `center` in which candidate points are
/// scattered. `min_range`/`max_range` are the inner/outer search radii.
#[derive(Debug, Clone, Copy)]
pub struct SearchRegion {
    pub center: ComplexNumber,
    pub min_range: f64,
    pub max_range: f64,
}


/// Gets a collection of focal points scattered in `search` and ranked by their
/// local detail, measured at the rendering extension `evaluation_extension`. Keeps the best
/// `num_of_points`.
pub fn get_focal_point_collection(
    search: SearchRegion,
    evaluation_extension: f64,
    iterator_parameter: &FractalConfig,
    num_of_points: usize,
    max_iterations: u16,
) -> FocalPointCollection {
    let iterator = iterator_parameter.into();
    let samples: Vec<_> = (0..num_of_points * SAMPLE_AMOUNT_FACTOR)
        .into_par_iter()
        .map_init(rand::rng, |rng, _| {
            let number = get_circle_random_complex_number(rng, search.min_range, search.max_range);
            get_potential_focal_point(
                number + search.center,
                evaluation_extension,
                iterator,
                max_iterations,
            )
        })
        .collect();

    let mut result = FocalPointCollection::new(num_of_points);
    result.append(&samples);
    result
}
