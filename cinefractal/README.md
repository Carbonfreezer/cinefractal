# Cinefractal

![License](https://img.shields.io/badge/license-MIT-green)
![Python](https://img.shields.io/badge/python-3.8%2B-blue)
![Built with](https://img.shields.io/badge/built%20with-Rust%20%2B%20PyO3-orange)

**Cinefractal** is a Python library for generating static and animated escape-time
fractals. The number crunching is written in Rust (via [PyO3](https://pyo3.rs/)
and [maturin](https://www.maturin.rs/)) and parallelised with
[rayon](https://github.com/rayon-rs/rayon), while the public API is plain Python
that returns NumPy arrays ready to hand off to Pillow or MoviePy.

The complete documentation can be found on [github pages](https://carbonfreezer.github.io/cinefractal/index.html).

The library has three main building blocks:

1. **Iteration field interface** — generates an iteration field or a colorized
   image of a fractal from a handful of parameters.
2. **Animation system** — a key-frame animation system that drives the iteration
   field interface over time, ready to render to MP4 with MoviePy.
3. **Focal (autofocus) system** — searches for interesting parts of a fractal
   using a variance-based heuristic; the points it finds can steer the animation.

## Gallery

|                                                           Mandelbrot                                                           | Julia | Burning Ship | Tricorn | Celtic |
|:------------------------------------------------------------------------------------------------------------------------------:|:-----:|:------------:|:-------:|:------:|
| ![Mandelbrot](https://raw.githubusercontent.com/Carbonfreezer/cinefractal/main/cinefractal/docs/images/mandelbrot_inferno.png) | ![Julia](https://raw.githubusercontent.com/Carbonfreezer/cinefractal/main/cinefractal/docs/images/inferno.png) | ![Burning Ship](https://raw.githubusercontent.com/Carbonfreezer/cinefractal/main/cinefractal/docs/images/burning_ship.png) | ![Tricorn](https://raw.githubusercontent.com/Carbonfreezer/cinefractal/main/cinefractal/docs/images/tricorn.png) | ![Celtic](https://raw.githubusercontent.com/Carbonfreezer/cinefractal/main/cinefractal/docs/images/celtic.png) |

## Installation

```bash
pip install cinefractal
```

Cinefractal requires Python 3.8+ and pulls in NumPy automatically. The bundled
examples additionally use, depending on what they do:

```bash
pip install pillow        # save/show images
pip install plotly pandas # visualize a raw iteration field
pip install moviepy       # render MP4 movies
```

## Quick start

Render the classic Mandelbrot image to a file:

```python
from PIL import Image
from cinefractal import IterationFieldInterface, ColorSystem

ifi = IterationFieldInterface()
ifi.set_numpy_extension(1024, 1280)        # rows (height), columns (width)
ifi.set_maximum_iterations(1000)
ifi.set_center_point(-1.0, 0.0)
ifi.set_extension(1.2)                      # smaller extension = deeper zoom
ifi.set_colorization_information(ColorSystem.cyclical(3.0), False)
ifi.set_colorization_log_strength(0.5)

color_field = ifi.get_color_field()         # numpy (rows, cols, 3) uint8
Image.fromarray(color_field).save("mandelbrot.png")
```

## The three building blocks

### Iteration field interface

`IterationFieldInterface` is the entry point for single images. It supports five
escape-time fractals:

| Fractal | Notes |
|---------|-------|
| Mandelbrot | the classic set |
| Julia | parametrised by a seed value (`set_fractal_seed_value`) |
| Burning Ship | |
| Tricorn | the "Mandelbar" |
| Celtic | |

All of them take an integer exponent (`>= 2`) used in the iteration formula.

The field is configured through small setter methods, each with a sensible
default:

| Parameter | Setter | Default |
|-----------|--------|---------|
| Image size (rows, columns) | `set_numpy_extension` | 1024 × 768 |
| Fractal type | `set_fractal_type` | Mandelbrot |
| Exponent | `set_fractal_exponent` | 2 |
| Julia seed value | `set_fractal_seed_value` | 0, 0 |
| Maximum iterations | `set_maximum_iterations` | 1000 |
| Extension (half-width of the view; smaller = more zoom) | `set_extension` | 1.5 |
| Center point | `set_center_point` | 0, 0 |

You can then query the result in three forms:

- `get_discrete_iteration_field()` → 2-D `uint16` array (iterations until escape)
- `get_continuous_iteration_field()` → 2-D `float32` array (smoothed)
- `get_color_field()` → 3-D `uint8` array `(rows, cols, 3)`, ready for Pillow

**Colorization** offers several base color schemes — `turbo`, `viridis`,
`magma`, `plasma`, `inferno`, `cool`, and `cyclical(repetition_factor)` — plus an
optional logarithmic strength that boosts the resolution of low iteration counts:

```python
from cinefractal import IterationFieldInterface, ColorSystem

ifi = IterationFieldInterface()
ifi.set_colorization_information(ColorSystem.viridis(), False)  # False = continuous
ifi.set_colorization_log_strength(0.5)
```

### Animation system

The animation system is key-frame based and drives an `IterationFieldInterface`
over time. You build the animation with an `AnimationRecorder`, freeze it into an
immutable `AnimationPlayer`, then ask the player to apply the animated state for a
given time before reading the color field.

```python
from cinefractal import AnimationRecorder, IterationFieldInterface, FractalType, ColorSystem

anim = AnimationRecorder(exponent=2, log_strength=2.0, max_iterations=1000)
anim.set_keyframe_fractal_seed_value(0.0, -0.8, 0.156)  # constant Julia seed
anim.set_keyframe_extension(6.0, 1e-4)                  # zoom in over 6 s
player = anim.get_animation_player()

ifi = IterationFieldInterface()
ifi.set_fractal_type(FractalType.julia())
ifi.set_colorization_information(ColorSystem.inferno(), False)

player.apply_animation(3.0, ifi)        # state at t = 3 s
frame = ifi.get_color_field()           # render that frame
```

Key frames exist for the exponent, Julia seed, render center point, extension
(zoom, interpolated in logarithmic space), maximum iterations, and color log
strength. Combined with MoviePy's `VideoClip`, this renders straight to MP4 —
see [`julia_zoom_movie.py`](cinefractal/python/julia_zoom_movie.py).

### Focal (autofocus) system

The focal system scatters candidate points across a search region and keeps the
most interesting ones, judged by the local variance of the iteration count.
`FocalSystemInterface` runs the (parallel) search and returns an immutable
`FocalPointResult`, which can hand the points back either unordered (with their
scores) or as an ordered path ready to drive an animation.

```python
from cinefractal import FocalSystemInterface, FractalType, PathGenerationCriterion

focus = FocalSystemInterface(max_iterations=300)
focus.set_fractal_type(FractalType.julia())
focus.set_fractal_seed_value(-0.8, 0.156)
focus.set_extension(2.0)                 # search radius
focus.set_evaluation_extension(1e-4)     # render extension the points are judged at

result = focus.create_collection_of_points(5)
path = result.get_path_from_start_point_with_distance(
    PathGenerationCriterion.short_path(), -1.5, 0.0
)   # list of (real, imag, cumulative_distance)
```

Note that the **search extension** (where candidates are scattered) is decoupled
from the **evaluation extension** (the extension you will actually render at;
smaller = deeper zoom). The cumulative distance returned by
`get_path_from_start_point_with_distance` is handy for timing a constant-speed
camera travel — see [`julia_autofocus_movie.py`](cinefractal/python/julia_autofocus_movie.py).

## Examples

All examples live in [`cinefractal/python/`](cinefractal/python/):

- **`iterationfield.py`** — query a raw iteration field and visualize it with plotly.
- **`mandelbrot.py`** — render a Mandelbrot image and save it to a file.
- **`julia_zoom_movie.py`** — a pre-scripted Julia zoom/pan rendered to MP4 with MoviePy.
- **`julia_autofocus_movie.py`** — autofocus picks interesting points and a movie tours them.

## Building from source

The project is a Cargo workspace; the Python package is the `cinefractal` crate.
From `cinefractal/`:

```bash
# 1. (Re)generate the type stub cinefractal.pyi from the Rust docstrings
cargo run --bin stub_gen

# 2. Build and install the extension module into the current environment
maturin develop --release
```

To build the HTML documentation (Sphinx), from `cinefractal/docs/`:

```bash
./make.bat html        # Windows
# or: make html        # Linux/macOS
```

## Project structure

This repository is a Rust workspace with three crates:

- **`fractal_core`** — the core fractal, animation, and focal-point engine (pure Rust).
- **`fractal_viz`** — a small native [macroquad](https://macroquad.rs/) demo used to
  test the core without Python.
- **`cinefractal`** — the PyO3/maturin bindings that expose the engine to Python.

## License

Licensed under the [MIT License](LICENSE). Copyright (c) 2026 Christoph Lürig.
