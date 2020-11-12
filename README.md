# Polaroid
![Travis](https://img.shields.io/travis/com/daggy1234/polaroid?logo=travis) ![Actions](https://img.shields.io/github/workflow/status/Daggy1234/polaroid/Continuous%20Integration?logo=github) ![License](https://img.shields.io/github/license/Daggy1234/polaroid?color=red) ![Wheel](https://img.shields.io/pypi/wheel/polaroid?color=blue&logo=pypi) ![Python](https://img.shields.io/pypi/pyversions/polaroid?color=yellow&logo=python&logoColor=yellow) ![Version](https://img.shields.io/pypi/v/polaroid) ![Chat](https://img.shields.io/discord/491175207122370581?color=gray&logo=discord) [![Codacy Badge](https://app.codacy.com/project/badge/Grade/bf1af7c59fd84144b5f29f8d8b27e5ba)](https://www.codacy.com/gh/Daggy1234/polaroid/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=Daggy1234/polaroid&amp;utm_campaign=Badge_Grade)
## Hyper fast image processing

This is a Work in Progress. DO NOT USE in production.

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

### Methods

```python
features = ['add_noise_rand', 'adjust_contrast', 'alter_blue_channel', 'alter_channel', 'alter_channels', 'alter_green_channel', 'alter_red_channel', 'apply_gradient', 'b_grayscale', 'blend', 'box_blur', 'brighten', 'colorize', 'crop', 'decompose_max', 'decompose_min', 'desaturate', 'detect_horizontal_lines', 'detect_vertical_lines', 'edge_detection', 'edge_one', 'emboss', 'filter', 'fliph', 'flipv', 'g_grayscale', 'gaussian_blur', 'grayscale', 'grayscale_human_corrected', 'grayscale_shades', 'horizontal_strips', 'identity', 'inc_brightness', 'invert', 'laplace',  'monochrome', 'noise_reduction', 'offset', 'offset_blue', 'offset_green', 'offset_red', 'pink_noise', 'prewitt_horizontal', 'primary', 'r_grayscale', 'remove_blue_channel', 'remove_green_channel', 'remove_red_channel', 'replace_backround', 'resize', 'rotate180', 'rotate270', 'rotate90', 'save', 'save_bytes', 'selective_desaturate', 'selective_hue_rotate', 'selective_lighten', 'selective_saturate', 'sepia', 'sharpen', 'single_channel_grayscale', 'sobel_horizontal', 'sobel_vertical', 'solarize', 'swap_channels', 'threshold', 'thumbnail', 'tint', 'unsharpen', 'vertical_strips', 'watermark']
#All available for Image
```

### Special Methods for `Image`

```python
from polaroid import Image
im = Image("meme.png")
print(repr(im))

#The `bytes` method is not implemented use
byt = im.save_bytes()
```