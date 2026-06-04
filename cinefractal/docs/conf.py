# Project meta data
project = "cinefractal"
copyright = "2026, Christoph Lürig"
author = "Christoph Lürig"

# Extensions
# Important: "myst_parser" instead "sphinx.ext.myst_parser"
extensions = [
    "myst_parser",
    "autodoc2",
]

# Autodoc2-Konfiguration for  .pyi / Rust-Stubs
autodoc2_packages = [
    {
        "path": "../cinefractal.pyi",  # Stub-Datei der kompilierten Rust-Erweiterung
        "auto_mode": True,
    },
]

# Docstrings are in  Markdown-Style (single backticks) -> parse with  MyST instead of RST p
autodoc2_docstring_parser_regexes = [
    (r".*", "myst"),
]

# cinefractal is the only package -> no additional api docs / index pages,
# instead we link the modul page directly in toctree.
autodoc2_index_template = None

# HTML-Theme (Read the Docs - blue page)
html_theme = "sphinx_rtd_theme"
