# Getting started

First, you need to install cinefractal in your environment.
```bash
pip install cinefractal
```

You will find several examples in the Python directory on GitHub. In order to execute them, you will need:
* **plotly** for the visualization of the direct iteration field
* **pandas**, which plotly relies on
* **Pillow** (imported as `PIL`) to generate the images,
* **MoviePy** to generate the mp4 movies.

You can install those in your environment with:
```bash
pip install plotly
pip install pandas
pip install Pillow
pip install moviepy
```

The following examples are included:
* **iterationfield.py**: This sample queries directly the iteration field from the system and visualizes it with plotly.
* **mandelbrot.py**: This sample generates a mandelbrot fractal rendering and saves it to a file.
* **julia_zoom_movie.py**: This sample generates a pre scripted movie of the Julia visualization demonstrating zooming and panning. The result is written to an MP4 file using MoviePy.
* **julia_autofocus_movie.py**: This is a showcase of the autofocus that selects some focus points and generates a movie of it.