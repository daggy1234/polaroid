# -*- coding: utf-8 -*-
"""
Polaroid
~~~~~~~~~~~~~~~~~~~
Hyper Fast and safe image manipulation library for python . Powered by rust.
:copyright: (c) 2020 Daggy
:license: MIT, see LICENSE for more details.
"""

from .polaroid import Image, Rgb, Gif, Rgba
from collections import namedtuple

__version__ = '0.3.2'
__author__ = "Daggy1234"
__license__ = "MIT"
__copyright__ = "Copyright 2021 Daggy1234"

__all__ = ["Image", "Rgb", "Gif", "Rgba"]

VersionInfo = namedtuple('VersionInfo',
                         'major minor micro releaselevel serial')

version_info = VersionInfo(major=0, minor=3, micro=2, releaselevel='development',
                           serial=0)
