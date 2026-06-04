//! This file contains the interface to the focal system.
//! It follows the same design philosophy as the other systems.
//! Constructor, default and a lot of micro setter methods.
//!

use crate::iteration_and_color::complex_number::ComplexNumber;
use crate::focal_system::focal_point::FocalPointCollection;
use crate::focal_system::quality_evaluator::{SearchRegion, get_focal_point_collection,
};
use crate::iteration_and_color::iteration_field_interface::{FractalConfig, FractalType};
use std::iter::once;

/// The interface is the configuration point for the focal system.
/// Its main purpose is to generate the [`FocalPointResult`].
#[derive(Debug, Clone)]
pub struct FocalSystemInterface {
    center_point: ComplexNumber,
    extension_min: f64,
    extension: f64,
    evaluation_extension: f64,
    maximum_iterations: u16,
    fractal_config: FractalConfig,
}

impl Default for FocalSystemInterface {
    fn default() -> Self {
        Self {
            center_point: ComplexNumber::default(),
            extension_min: 0.0,
            extension: 1.5,
            // Defaults to the search extension, reproducing the previous coupled
            // behaviour until the caller sets a dedicated evaluation scale.
            evaluation_extension: 1.5,
            maximum_iterations: 1000,
            fractal_config: FractalConfig::default(),
        }
    }
}

/// Enforces the search-radius invariant `0 <= extension_min < extension`.
/// A violated invariant would make `get_circle_random_complex_number` take the
/// square root of a negative number, producing NaN candidate positions.
fn validate_search_radii(extension_min: f64, extension: f64) -> Result<(), String> {
    if !(extension_min >= 0.0 && extension_min < extension) {
        return Err(format!(
            "search radii must satisfy 0 <= extension_min < extension, \
             got extension_min={extension_min}, extension={extension}"
        ));
    }
    Ok(())
}

impl FocalSystemInterface {
    /// Sets the fractal type to render. The exponent and Julia seed are set
    /// separately via [`set_fractal_exponent`](Self::set_fractal_exponent) and
    /// [`set_fractal_seed_value`](Self::set_fractal_seed_value).
    pub fn set_fractal_type(&mut self, fractal_type: FractalType) {
        self.fractal_config.fractal_type = fractal_type;
    }

    /// Sets the iteration exponent. Should be >= 2; this is enforced at the
    /// public entry points (the Python binding and the animation key frame
    /// setter), so callers here are trusted.
    pub fn set_fractal_exponent(&mut self, exponent: u32) {
        self.fractal_config.exponent = exponent;
    }

    /// Sets the Julia seed (start) value. Only relevant for the Julia set;
    /// ignored by the other fractal types.
    pub fn set_fractal_seed_value(&mut self, real: f64, imag: f64) {
        self.fractal_config.seed_real = real;
        self.fractal_config.seed_imag = imag;
    }

    /// Sets the maximum number of iterations for the project.
    pub fn set_maximum_iterations(&mut self, maximum_iterations: u16) {
        self.maximum_iterations = maximum_iterations;
    }

    /// Sets the center point for iteration.
    pub fn set_center_point(&mut self, real: f64, imag: f64) {
        self.center_point = ComplexNumber::new(real, imag);
    }

    /// Sets the (outer) search extension: the maximum distance from the search
    /// center at which candidate points are scattered. Must stay strictly greater
    /// than the current minimum extension, preserving `0 <= extension_min < extension`.
    pub fn set_extension(&mut self, extension: f64) -> Result<(), String> {
        validate_search_radii(self.extension_min, extension)?;
        self.extension = extension;
        Ok(())
    }

    /// Sets the minimum extension from the searched center. Must satisfy
    /// `0 <= extension_min < extension`.
    pub fn set_extension_min(&mut self, min: f64) -> Result<(), String> {
        validate_search_radii(min, self.extension)?;
        self.extension_min = min;
        Ok(())
    }

    /// Sets the evaluation extension: the rendering extension (the view's
    /// half-width in the complex plane) at which the local detail of a candidate
    /// point is measured. It should match the extension at which the points will
    /// later be rendered/viewed -- a smaller value means a deeper zoom. This is
    /// independent of the search radius set via [`set_extension`](Self::set_extension),
    /// which is usually much larger.
    pub fn set_evaluation_extension(&mut self, evaluation_extension: f64) {
        self.evaluation_extension = evaluation_extension;
    }


    /// Creates a collection of focal points according to the current parametrization.
    pub fn create_collection_of_points(&self, num_of_points: usize) -> FocalPointResult {
        let collection = get_focal_point_collection(
            SearchRegion {
                center: self.center_point,
                min_range: self.extension_min,
                max_range: self.extension,
            },
            self.evaluation_extension,
            &self.fractal_config,
            num_of_points,
            self.maximum_iterations,
        );
        FocalPointResult {
            focal_point_collection: collection,
        }
    }
}

/// This is the result structure we get from requesting a focal point.
pub struct FocalPointResult {
    focal_point_collection: FocalPointCollection,
}

/// Flags the path combination criterion on how focal points are arranged in a sequence.
#[derive(Clone, Copy)]
pub enum PathGenerationCriterion {
    /// We want to generate a reasonably short path (greedy TSP approximation)
    ShortPath,
    /// We want to generate a path with descending values (most interesting point to least interesting one).
    DescendingValuePath,
}

impl FocalPointResult {
    /// Simply gets all points in the focal point result without any specific order annotated with the evaluation of the point in third dimension.
    pub fn get_focal_points_with_evaluation(&self) -> Vec<(f64, f64, f64)> {
        self.focal_point_collection
            .get_iterator()
            .map(|x| (x.point.real, x.point.imag, x.evaluation))
            .collect()
    }

    /// Gets the best path with the indicated criterion.
    pub fn get_path_from_start_point(
        &self,
        criterion: PathGenerationCriterion,
        start_real: f64,
        start_imag: f64,
    ) -> Vec<(f64, f64)> {
        let start = ComplexNumber::new(start_real, start_imag);
        use PathGenerationCriterion::*;
        let inbetween = match criterion {
            ShortPath => self.focal_point_collection.get_short_path(start),
            DescendingValuePath => self.focal_point_collection.get_descending_path(),
        };
        once((start_real, start_imag))
            .chain(inbetween.into_iter().map(|x| (x.point.real, x.point.imag)))
            .collect()
    }

    /// Gets a measured sequence of the best path according to the optimization criterion. It hands back
    /// frames consisting of the point (for the animation) and the cumulative distance travelled from the
    /// start. These values may be used for timing animations.
    pub fn get_path_from_start_point_with_distance(
        &self,
        criterion: PathGenerationCriterion,
        start_real: f64,
        start_imag: f64,
    ) -> Vec<(f64, f64, f64)> {
        // First we get the list with the positions including the start position.
        let pos_list = self.get_path_from_start_point(criterion, start_real, start_imag);
        pos_list
            .iter()
            .scan((pos_list[0], 0.0f64), |((x, y), d), (nx, ny)| {
                *d += ((*nx - *x).powi(2) + (*ny - *y).powi(2)).sqrt();
                *x = *nx;
                *y = *ny;
                Some((*nx, *ny, *d))
            })
            .collect()
    }
}
