from moviepy import VideoClip

from cinefractal import (
    AnimationRecorder,
    ColorSystem,
    FractalType,
    IterationFieldInterface,
)

WIDTH = 640
HEIGHT = 480
FPS = 10
PANNING = 1.0 # The time we need to pan to the destination
ZOOMIN = 4.0  # Length for zoomin in.
ZOOMOUT = 1.5 # Length for zooming out.
FINALPAN = 1.0 # Length for the final panning phase.
SEAHORSE = (-0.7436, 0.1318) # Position of the seahorse valley.

# --- Build the animation ---------------------------------------------------
# The constructor seeds time 0. Defaults mirror IterationFieldInterface, so we
# only override what we animate or want different from the start.
animation = AnimationRecorder(
    exponent=2,
    # log color scaling (mapped via exp_m1 internally), so the fast-escaping
    log_strength=3.0,
    max_iterations=10000,
)

# Start at the general oversight.
animation.set_keyframe_render_center_point(0, -1.0, 0.0)
animation.set_keyframe_extension(0, 1.2)
animation.set_keyframe_render_center_point(PANNING, *SEAHORSE)
animation.set_keyframe_extension(ZOOMIN, 1e-4)
animation.set_keyframe_render_center_point(ZOOMIN + ZOOMOUT, *SEAHORSE)
animation.set_keyframe_extension(ZOOMIN + ZOOMOUT, 1.2)
animation.set_keyframe_render_center_point(ZOOMIN + ZOOMOUT + FINALPAN, -1.0, 0.0)


player = animation.get_animation_player()
duration = player.max_time()


# --- Configure the field interface (type / size / colors set once) ---------
ifi = IterationFieldInterface()
ifi.set_numpy_extension(HEIGHT, WIDTH)
# Only the fractal *type* needs to be set here; exponent and seed value are
# driven by the animation every frame.
ifi.set_fractal_type(FractalType.mandelbrot())
ifi.set_colorization_information(ColorSystem.cyclical(3.0), False)


def make_frame(t):
    """Render the frame at time `t` (seconds)."""
    player.apply_animation(t, ifi)
    # (rows, cols, 3) uint8 -- the exact frame layout moviepy wants.
    return ifi.get_color_field()


clip = VideoClip(make_frame, duration=duration)

output_path = "sea_horse.gif"
print(f"Rendering {duration:.1f}s at {WIDTH}x{HEIGHT}, {FPS} fps -> {output_path}")
clip.write_gif(output_path, fps=FPS)
print(f"Done: {output_path}")

