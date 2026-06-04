//! This module contains the basic concepts for fractal escapes.

use rayon::iter::ParallelIterator;
use crate::iteration_and_color::complex_number::ComplexNumber;
use crate::iteration_and_color::fractal_collection::{AnyEscapeIteration, EscapeIteration};
use rayon::iter::IntoParallelIterator;
use rayon::iter::IndexedParallelIterator;

/// The extension of the window we represent, has a lot of helper functions for dimension
/// calculations.
#[derive(Debug, Clone, Copy)]
pub struct WindowExtension {
    x_dim: i32,
    y_dim: i32,
}

impl WindowExtension {
    /// Creates a window extension from its width (`x_dim`) and height (`y_dim`) in pixels.
    pub fn new(x_dim: i32, y_dim: i32) -> WindowExtension {
        WindowExtension { x_dim, y_dim }
    }

    /// Gets the smaller of width and height.
    fn get_smallest_size(&self) -> f64 {
        self.x_dim.min(self.y_dim) as f64
    }

    /// Gets the amount of pixels in the window.
    fn get_amount_of_pixels(&self) -> i32 {
        self.x_dim * self.y_dim
    }

    /// Gets the numpy-style shape in (rows, columns).
    pub fn get_numpy_shape(&self) -> (usize, usize) {
        (self.y_dim as usize, self.x_dim as usize)
    }

    /// Gets the numpy-style shape with the trailing color channel: (rows, columns, 3).
    pub fn get_numpy_shape_color(&self) -> (usize, usize, usize) {
        (self.y_dim as usize, self.x_dim as usize, 3)
    }
}

/// This method encapsulates the iteration scheme to find out if and when the iterator
/// diverges. Internally a monomorphized version is generated for every iterator type.
fn run_iterator(
    mut system: impl EscapeIteration,
    pixel_pos: ComplexNumber,
    maximum_iterations: u16,
) -> (u16, f64) {
    system.set_start_pixel_in_number_pane(pixel_pos);
    let bail_out = system.get_squared_bail_out_radius(); // einmal, nicht pro Iteration
    let mut iter_value = system.get_iteration_start_value();
    let mut counter = 0u16;
    loop {
        let sq_mag = iter_value.sq_magnitude();
        if counter >= maximum_iterations || sq_mag > bail_out {
            return (counter, sq_mag);
        }
        counter += 1;
        iter_value = system.get_iteration_step(iter_value);
    }
}

/// Finds the iteration values (number of iterations and squared escape magnitude) for a given complex number.
pub fn find_iteration_on_complex_number(
    system: AnyEscapeIteration,
    probing_point: ComplexNumber,
    maximum_iterations: u16,
) -> (u16, f64) {
    match system {
        AnyEscapeIteration::Mandelbrot(m) => run_iterator(m, probing_point, maximum_iterations),
        AnyEscapeIteration::Julia(j) => run_iterator(j, probing_point, maximum_iterations),
        AnyEscapeIteration::BurningShip(b) => run_iterator(b, probing_point, maximum_iterations),
        AnyEscapeIteration::Celtic(c) => run_iterator(c, probing_point, maximum_iterations),
        AnyEscapeIteration::Tricorn(t) => run_iterator(t, probing_point, maximum_iterations),
    }
}

/// Performs the iteration estimation of a single pixel. Returns the amount of iterations till divergence and
/// the eventual remaining square value. Here the dispatch between the different fractal types happens.
fn iterate_pixel(
    pixel_index: i32,
    window: &WindowExtension,
    system: AnyEscapeIteration,
    step_increment: f64,
    center_point: ComplexNumber,
    maximum_iterations: u16,
) -> (u16, f64) {
    let y_pos = pixel_index / window.x_dim - window.y_dim / 2;
    let x_pos = pixel_index % window.x_dim - window.x_dim / 2;
    let mut pixel_pos =
        ComplexNumber::new(x_pos as f64 * step_increment, y_pos as f64 * step_increment);
    pixel_pos += center_point;
    find_iteration_on_complex_number(system, pixel_pos, maximum_iterations)
}

/// Computes a discrete iteration field (iterations needed per position until divergence).
pub fn get_iteration_field_discrete(
    window: WindowExtension,
    system: AnyEscapeIteration,
    center_point: ComplexNumber,
    extension: f64,
    maximum_iterations: u16,
) -> impl IndexedParallelIterator<Item = u16> {
    let step_increment = extension / (window.get_smallest_size() * 0.5);
    (0..window.get_amount_of_pixels())
        .into_par_iter()
        .map(move |x| {
            let (counter, _) = iterate_pixel(
                x,
                &window,
                system,
                step_increment,
                center_point,
                maximum_iterations,
            );
            counter // kein sqrt/ln/ln
        })
}

/// Gets a continuous iteration field, the same as the discrete one but blending between the
/// discrete steps to get a smooth transition.
pub fn get_iteration_field_continuous(
    window: WindowExtension,
    system: AnyEscapeIteration,
    center_point: ComplexNumber,
    extension: f64,
    maximum_iterations: u16,
) -> impl IndexedParallelIterator<Item = f32> {
    let step_increment = extension / (window.get_smallest_size() * 0.5);
    let inv_log_exp = 1.0 / system.get_smoothing_degree().ln();
    (0..window.get_amount_of_pixels())
        .into_par_iter()
        .map(move |x| {
            let (counter, sq_mag) = iterate_pixel(
                x,
                &window,
                system,
                step_increment,
                center_point,
                maximum_iterations,
            );
            if counter >= maximum_iterations {
                counter as f32
            } else {
                counter as f32 - (sq_mag.sqrt().ln().ln() * inv_log_exp) as f32
            }
        })
}
