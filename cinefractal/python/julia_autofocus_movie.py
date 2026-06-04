"""
Example: autofocus + animated Julia tour rendered to an MP4 with moviepy.

This combines the autofocus logic of the native `fractal_viz` demo with the
movie-rendering structure of `julia_zoom_movie.py`:

  1. The focal system scatters candidates in a coarse search region and keeps
     the most interesting ones (highest local variance), judged at a deep
     evaluation scale.
  2. The kept points are ordered into a short tour, annotated with the
     cumulative travel distance.
  3. That distance drives the timing of an AnimationRecorder: the camera pans to
     each point (at constant speed), then zooms in to detail scale and back out
     before travelling on.
  4. moviepy renders the resulting AnimationPlayer frame by frame.

Requirements:
    pip install moviepy
    maturin develop --release   # to build/install cinefractal

Run:
    python julia_autofocus_movie.py
"""

from moviepy import VideoClip

from cinefractal import (
    AnimationRecorder,
    ColorSystem,
    FocalSystemInterface,
    FractalType,
    IterationFieldInterface,
    PathGenerationCriterion,
)

# --- Render settings -------------------------------------------------------
WIDTH = 1280
HEIGHT = 720
FPS = 30

# --- Fractal / autofocus settings (mirrors the fractal_viz demo) -----------
JULIA_SEED = (-0.8, 0.156)
MAX_ITERATIONS = 300

COARSE_SCALE = 1.5          # extension we hover at between zooms
COARSE_SEARCH_SCALE = 2.0   # radius we scatter search candidates in
DETAIL_SCALE = 1e-4         # extension we dive to at each point (smaller = deeper zoom)

NUM_FOCAL_POINTS = 5       # how many interesting points to keep / visit
START_POINT = (-1.5, 0.0)   # where the camera tour starts

# --- Timing ----------------------------------------------------------------
TRAVERSAL_SECONDS = 4.0     # total time spent panning across the whole tour
ZOOM_TIME = 4.0             # seconds per zoom phase (in and out separately)


def build_animation():
    """Run the autofocus search and turn the resulting tour into a player."""
    animation = AnimationRecorder(
        exponent=2,
        log_strength=2.0,
        max_iterations=MAX_ITERATIONS,
        extension=COARSE_SCALE,
        julia_start_real=JULIA_SEED[0],
        julia_start_imag=JULIA_SEED[1],
    )

    # Configure and run the focal search.
    focal = FocalSystemInterface(max_iterations=MAX_ITERATIONS)
    focal.set_fractal_type(FractalType.julia())
    focal.set_fractal_seed_value(*JULIA_SEED)
    focal.set_extension(COARSE_SEARCH_SCALE)
    # Judge each candidate at the depth we actually dive to, decoupled from the
    # search radius.
    focal.set_evaluation_extension(DETAIL_SCALE)

    result = focal.create_collection_of_points(NUM_FOCAL_POINTS)
    path = result.get_path_from_start_point_with_distance(
        PathGenerationCriterion.short_path(), *START_POINT
    )

    # Scale travel distance to wall-clock time so the pan runs at constant speed.
    end_dist = path[-1][2]
    time_scale = TRAVERSAL_SECONDS / end_dist if end_dist > 0.0 else 0.0

    time_passed = 0.0
    for real, imag, dist in path:
        time_passed += dist * time_scale
        animation.set_keyframe_render_center_point(time_passed, real, imag)

        # The start point only seeds the pan; don't zoom there.
        if dist <= 0.0:
            continue

        # Hold the coarse scale on arrival, dive to detail, then come back out.
        animation.set_keyframe_extension(time_passed, COARSE_SCALE)
        time_passed += ZOOM_TIME
        animation.set_keyframe_extension(time_passed, DETAIL_SCALE)
        time_passed += ZOOM_TIME
        animation.set_keyframe_extension(time_passed, COARSE_SCALE)
        # Re-pin the center so we don't drift off target while zooming.
        animation.set_keyframe_render_center_point(time_passed, real, imag)

    return animation.get_animation_player()


# --- Build animation and configure the field interface ---------------------
animation = build_animation()
duration = animation.max_time()

ifi = IterationFieldInterface()
ifi.set_numpy_extension(HEIGHT, WIDTH)
# Only the type is set here; exponent and seed are driven by the animation.
ifi.set_fractal_type(FractalType.julia())
ifi.set_colorization_information(ColorSystem.inferno(), False)


def make_frame(t):
    """Render the frame at time `t` (seconds)."""
    animation.apply_animation(t, ifi)
    # (rows, cols, 3) uint8 -- the exact frame layout moviepy wants.
    return ifi.get_color_field()


clip = VideoClip(make_frame, duration=duration)

output_path = "julia_autofocus.mp4"
print(f"Rendering {duration:.1f}s at {WIDTH}x{HEIGHT}, {FPS} fps -> {output_path}")
clip.write_videofile(output_path, fps=FPS)
print(f"Done: {output_path}")
