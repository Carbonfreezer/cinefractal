"""
Example: Mandelbrot fractal with a cyclical color scheme.

Requirements:
    pip install Pillow

"""

from PIL import Image
from cinefractal import IterationFieldInterface, ColorSystem

WIDTH = 1280
HEIGHT = 1024
IMAGE_NAME = "mandelbrot.png"

ifi = IterationFieldInterface()
ifi.set_numpy_extension(HEIGHT, WIDTH)
ifi.set_maximum_iterations(1000)
ifi.set_center_point(-1.0, 0.0)
ifi.set_extension(1.2)
ifi.set_colorization_information(ColorSystem.cyclical(3.0), False)
ifi.set_colorization_log_strength(0.5)

print("Computing mandelbrot fractal...")
color_field = ifi.get_color_field()

image = Image.fromarray(color_field)
image.save(IMAGE_NAME)
image.show()
print(f"File saved at {IMAGE_NAME} with resolution {WIDTH}x{HEIGHT}")
