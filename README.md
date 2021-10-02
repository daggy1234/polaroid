# Polaroid

[![Deployment](https://github.com/Daggy1234/polaroid/actions/workflows/publish.yml/badge.svg)](https://github.com/Daggy1234/polaroid/actions/workflows/publish.yml) ![Actions](https://img.shields.io/github/workflow/status/Daggy1234/polaroid/Continuous%20Integration?logo=github) ![License](https://img.shields.io/github/license/Daggy1234/polaroid?color=red) ![Wheel](https://img.shields.io/pypi/wheel/polaroid?color=blue&logo=pypi) ![Python](https://img.shields.io/pypi/pyversions/polaroid?color=yellow&logo=python&logoColor=yellow) ![Version](https://img.shields.io/pypi/v/polaroid) ![Rust Report](https://rust-reportcard.xuri.me/badge/github.com/daggy1234/polaroid) ![Chat](https://img.shields.io/discord/491175207122370581?color=gray&logo=discord) [![Codacy Badge](https://app.codacy.com/project/badge/Grade/bf1af7c59fd84144b5f29f8d8b27e5ba)](https://www.codacy.com/gh/Daggy1234/polaroid/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=Daggy1234/polaroid&amp;utm_campaign=Badge_Grade)

## Hyper fast image processing

This is a Work in Progress.

For all the examples, meme.png is used. Please replace that with whatever Image you use

### Via File System

```python
from polaroid import Image
im = Image("meme.png")
im.solarize()
im.save("solar.png")
```

### Using Bytes

```python
# Just an example use any library to supply bytes
from polaroid import Image
import requests
byt = requests.get("https://dagpi.xyz/dagpi.png").content
im = Image(byt)
im.filter("dramatic")
ret_byt = im.save_bytes()
```

### Properties

```python
from polaroid import Image
im = Image("meme.png")
h = im.height
w = im.weight
wi,hei = im.size
image_format = im.format
mode = im.mode
```

### Using the RGB class for coloring

```py
from polaroid import Image, Rgb
im = Image("meme.png")
# Color Rgb 
color = Rgb(78, 93, 148)
im.color(color)

# Now Save

```

### Methods

#### Image Methods

```python
['add_noise_rand', 'adjust_contrast', 'alter_blue_channel', 'alter_channel', 'alter_channels', 'alter_green_channel', 'alter_red_channel', 'apply_gradient', 'b_grayscale', 'blend', 'box_blur', 'brighten', 'color', 'color_no_grayscale', 'colorize', 'crop', 'decompose_max', 'decompose_min', 'desaturate', 'detect_horizontal_lines', 'detect_vertical_lines', 'edge_detection', 'edge_one', 'emboss', 'filter', 'fliph', 'flipv',  'g_grayscale', 'gaussian_blur', 'gradient', 'grayscale', 'grayscale_human_corrected', 'grayscale_shades',  'hog', 'horizontal_strips', 'identity', 'inc_brightness', 'invert', 'laplace', 'liquid_rescale',  'monochrome', 'noise_reduction', 'offset', 'offset_blue', 'offset_green', 'offset_red', 'oil', 'pink_noise', 'prewitt_horizontal', 'primary', 'r_grayscale', 'remove_blue_channel', 'remove_green_channel', 'remove_red_channel', 'replace_backround', 'resize', 'rotate180', 'rotate270', 'rotate90', 'save', 'save_base_64', 'save_bytes', 'save_jpeg_bytes', 'selective_desaturate', 'selective_hue_rotate', 'selective_lighten', 'selective_saturate', 'sepia', 'sharpen', 'single_channel_grayscale', 'sobel_horizontal', 'sobel_vertical', 'solarize', 'swap_channels', 'threshold', 'thumbnail', 'tint', 'unsharpen', 'vertical_strips', 'watermark']
#All available for Image
```

#### Rgb Methods

```py

```

### Special Methods for `Image`

```python
from polaroid import Image
im = Image("meme.png")
print(repr(im))

#The `bytes` method is not implemented use
byt = im.save_bytes()

# Save a jpeg

im.save_jpeg(quaility: int)

byt = im.save_jpeg_bytes(quality: int)
```

## Checklist

* [x] initial release

* [x] automated releases and pypi wheels

* [x] wheels for alpine linux

* [ ] full documentation

* [ ] benchmarks

* [ ] image draw features

* [ ] Stable release and promotion
