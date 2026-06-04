"""
This sample requests the iteration field directly and visualizes it with plotly.

Requirements:
    pip install plotly pandas

"""

import plotly.express as px
from cinefractal import IterationFieldInterface, FractalType

ifi = IterationFieldInterface()
ifi.set_fractal_type(FractalType.mandelbrot())
ifi.set_fractal_exponent(2)
ifi.set_maximum_iterations(20)
ifi.set_center_point(-0.5, 0.0)
ifi.set_extension(1.5)

iter_discrete = ifi.get_discrete_iteration_field()
fig = px.imshow(iter_discrete, color_continuous_scale='Viridis')
fig.show()

iter_continuous = ifi.get_continuous_iteration_field()
fig = px.imshow(iter_continuous, color_continuous_scale='Viridis')
fig.show()