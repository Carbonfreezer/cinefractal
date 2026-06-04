"""
Example: render an animated Julia zoom to an MP4 file using moviepy.

A Julia set with a fixed start value (-0.8, 0.156) animated in three segments:
zoom in (radius 1.5 -> 1e-4), zoom back out (-> 0.1), then pan to the right
while holding the radius. Demonstrates several animation tracks (radius and
center point) with hold key frames.

The animation system mutates the IterationFieldInterface each frame; we then ask
the interface for its color field, which is already a (rows, cols, 3) uint8
array -- exactly the frame format moviepy expects.

Requirements:
    pip install moviepy

"""

from moviepy import VideoClip

from cinefractal import (
    AnimationRecorder,
    ColorSystem,
    FractalType,
    IterationFieldInterface,
)

WIDTH = 1280
HEIGHT = 720
FPS = 30
SEGMENT = 6.0  # seconds per animation segment; total duration is 3 * SEGMENT

# --- Build the animation ---------------------------------------------------
# The constructor seeds time 0. Defaults mirror IterationFieldInterface, so we
# only override what we animate or want different from the start.
animation = AnimationRecorder(
    exponent=2,
    # log color scaling (mapped via exp_m1 internally), so the fast-escaping
    # zoom stays colorful. exp_m1(2.0) ~= 6.4, a moderate strength.
    log_strength=2.0,
    max_iterations=1000,
)
# Constant fractal seed (Julia start) value (set as the seed at t = 0).
animation.set_keyframe_fractal_seed_value(0.0, -0.8, 0.156)
# Segment 1 (0 .. SEGMENT): logarithmic zoom in, extension 1.5 -> 1e-4.
animation.set_keyframe_extension(SEGMENT, 1e-4)
# Segment 2 (SEGMENT .. 2*SEGMENT): zoom back out to 0.1.
animation.set_keyframe_extension(SEGMENT * 2, 0.1)
# Hold the center at the origin through both zoom segments.
animation.set_keyframe_render_center_point(SEGMENT * 2, 0.0, 0.0)
# Segment 3 (2*SEGMENT .. 3*SEGMENT): pan to the right (extension held at 0.1).
animation.set_keyframe_render_center_point(3 * SEGMENT, 1.0, 0.0)
animation = animation.get_animation_player()
duration = animation.max_time()

# --- Configure the field interface (type / size / colors set once) ---------
ifi = IterationFieldInterface()
ifi.set_numpy_extension(HEIGHT, WIDTH)
# Only the fractal *type* needs to be set here; exponent and seed value are
# driven by the animation every frame.
ifi.set_fractal_type(FractalType.julia())
ifi.set_colorization_information(ColorSystem.inferno(), False)


def make_frame(t):
    """Render the frame at time `t` (seconds)."""
    animation.apply_animation(t, ifi)
    # (rows, cols, 3) uint8 -- the exact frame layout moviepy wants.
    return ifi.get_color_field()


clip = VideoClip(make_frame, duration=duration)

output_path = "julia_zoom.mp4"
print(f"Rendering {duration:.1f}s at {WIDTH}x{HEIGHT}, {FPS} fps -> {output_path}")
clip.write_videofile(output_path, fps=FPS)
print(f"Done: {output_path}")