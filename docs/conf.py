# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.


import shutil
import datetime
import re
import inspect
import os
import sys
import polaroid
import semantic_version

# import os
# import sys
# sys.path.insert(0, os.path.abspath('.'))


docssrc_dir = os.path.abspath(os.path.join(__file__, ".."))
project_dir = os.path.dirname(docssrc_dir)
sys.path.insert(0, os.path.abspath('..'))
sys.path.append(os.path.abspath('extensions'))
# -- Project information -----------------------------------------------------

project = 'polaroid'
copyright = '2021, Daggy1234'
author = 'Daggy1234'
year = datetime.date.today().year

# The full version, including alpha/beta/rc tags
with open('../polaroid/__init__.py') as f:
    version = re.search(r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]', f.read(),
                        re.MULTILINE).group(1)
release = version

# -- Setup -------------------------------------------------------------------


def setup(app):
    # Add custom signature inspector support *argument-clinic* signatures.
    def inspector(app, what, name, obj, options, signature, return_annotation):
        if signature is not None:
            return signature, return_annotation
        try:
            sig = inspect.signature(obj)
            return str(sig), return_annotation
        except (ValueError, TypeError):
            return None, return_annotation

    app.connect('autodoc-process-signature', inspector)

# -- General configuration ---------------------------------------------------


# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.doctest",
    "sphinx.ext.intersphinx",
    "sphinx.ext.napoleon",
    "sphinx.ext.todo",
    "sphinx.ext.coverage",
    "sphinx.ext.ifconfig",
    "sphinx.ext.viewcode",
    "sphinx.ext.githubpages",
    "nbsphinx"
]

# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

source_suffix = [".rst", ".md"]

master_doc = "index"

pygments_style = "emacs"

default_role = "py:obj"

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'sphinx_book_theme'
html_title = "polaroid"

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ['_static']

# -- Extension configuration -------------------------------------------------

# -- Options for imgmath extension -------------------------------------------

imgmath_image_format = "svg"

# -- Options for napoleon extension ------------------------------------------

napoleon_include_init_with_doc = True
napoleon_include_special_with_doc = True
napoleon_include_private_with_doc = True
napoleon_use_admonition_for_examples = True
napoleon_use_admonition_for_notes = True
napoleon_use_admonition_for_references = True
napoleon_use_rtype = False

# -- Options for autodoc extension -------------------------------------------

autoclass_content = "class"

# -- Options for intersphinx extension ---------------------------------------

# Example configuration for intersphinx: refer to the Python standard library.
intersphinx_mapping = {
    "python": ("https://docs.python.org/3/", None),
}

# -- Options for todo extension ----------------------------------------------

# If true, `todo` and `todoList` produce output, else they produce nothing.
todo_include_todos = True


html_theme_options = {
    "toc_title": f"Polaroid {version}",
    "repository_url": "https://github.com/daggy1234/polaroid",
    "use_issues_button": True,
}
