//! The base module for the raw Python maturin implementation of the fractal system.
mod py_animation_system;
mod py_color_system;
mod py_focal_system;
mod py_fractal_type;
mod py_iteration_field_interface;

use py_animation_system::{PyAnimationPlayer, PyAnimationRecorder};
use py_color_system::PyColorSystem;
use py_focal_system::{PyFocalPointResult, PyFocalSystemInterface, PyPathGenerationCriterion};
use py_fractal_type::PyFractalType;
use py_iteration_field_interface::PyIterationFieldInterface;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

// Build & stub workflow:
// 1. Regenerate cinefractal.pyi (incl. docstrings) from the Rust sources:
//      cargo run --bin stub_gen
// 2. Build/install the extension module:
//      maturin develop --release
// 3. Generate HTML documentation:
//      ./make.bat html in docs 
// The stub is now produced by pyo3-stub-gen, so the previous manual steps
// (maturin generate-stubs, hand-adding _typeshed/Incomplete) are obsolete.
// CI:
//   maturin generate-ci github


/// This module provides functionality to generate images and movies from 
/// fractals. It has three main building blocks. One for generating fractal images,
/// one for animation and one for finding interesting focus points.
#[pymodule]
mod cinefractal {
    #[pymodule_export]
    use super::PyAnimationPlayer;
    #[pymodule_export]
    use super::PyAnimationRecorder;
    #[pymodule_export]
    use super::PyColorSystem;
    #[pymodule_export]
    use super::PyFocalPointResult;
    #[pymodule_export]
    use super::PyFocalSystemInterface;
    #[pymodule_export]
    use super::PyPathGenerationCriterion;
    #[pymodule_export]
    use super::PyFractalType;
    #[pymodule_export]
    use super::PyIterationFieldInterface;
}

// The `///` doc comment above `mod cinefractal` only sets the runtime
// `__doc__`; it is NOT picked up by the stub generator. IDEs read the module
// docstring from the static `cinefractal.pyi`, so it must be registered
// explicitly with `module_doc!` to appear there (and thus on hover).
pyo3_stub_gen::module_doc!(
    "cinefractal",
    "This module provides functionality to generate images and movies from \
     fractals. It has three main building blocks. One for generating fractal \
     images, one for animation and one for finding interesting focus points. \
     For details see https://github.com/Carbonfreezer/cinefractal."
);

// Gathers the stub information registered by the `#[gen_stub_*]` macros and
// exposes it as `cinefractal::stub_info()`, which `src/bin/stub_gen.rs` calls.
// The module name is read from pyproject.toml ([project] name = "cinefractal").
define_stub_info_gatherer!(stub_info);
