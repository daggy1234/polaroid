import os
import sys
import re
from setuptools import setup
from setuptools_rust import Binding, RustExtension
from setuptools.command.sdist import sdist as SdistCommand

with open('polaroid/__init__.py') as f:
    version = re.search(r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]', f.read(), re.MULTILINE).group(1)

setup_requires = ['setuptools-rust>=0.9.2']
install_requires = []

setup(name='polaroid',
      author="Daggy1234",
      version=version,
      description="Hyper Fast and safe image manipulation library for python . Powered by rust.",
      long_description=open("README.md").read(),
      long_description_content_type="text/markdown",
      license="MIT",
      url="https://github.com/Daggy1234/polaroid",
      project_urls={
          "Repository": "https://github.com/Daggy1234/polaroid",
          "Issue tracker": "https://github.com/Daggy1234/polaroid/issues",
          "Chat": "https://server.daggy.tech"
      },
      classifiers=[
          "Intended Audience :: Developers",
          'License :: OSI Approved :: MIT License',
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Programming Language :: Python',
          'Programming Language :: Rust',
          "Operating System :: OS Independent",
          'Natural Language :: English',
          "Programming Language :: Python :: 3.6",
          "Programming Language :: Python :: 3.7",
          "Programming Language :: Python :: 3.8",
          "Programming Language :: Python :: 3.9",
          'Topic :: Internet',
          'Topic :: Software Development :: Libraries',
          'Topic :: Software Development :: Libraries :: Python Modules',
          'Topic :: Utilities'
      ],
      rust_extensions=[
          RustExtension('polaroid.polaroid', 'Cargo.toml', binding=Binding.PyO3)],
      setup_requires=setup_requires,
      include_package_data=True,
      packages=['polaroid'],
      zip_safe=False,
      python_requires='>=3.6'
      )