# Polaroid
![Travis](https://img.shields.io/travis/com/daggy1234/polaroid?logo=travis) ![Actions](https://img.shields.io/github/workflow/status/Daggy1234/polaroid/Continuous%20Integration?logo=github) ![License](https://img.shields.io/github/license/Daggy1234/polaroid?color=red) ![Wheel](https://img.shields.io/pypi/wheel/polaroid?color=blue&logo=pypi) ![Python](https://img.shields.io/pypi/pyversions/polaroid?color=yellow&logo=python&logoColor=yellow) ![Version](https://img.shields.io/pypi/v/polaroid) ![Chat](https://img.shields.io/discord/491175207122370581?color=gray&logo=discord)
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
methods = ['adjust_contrast', 'box_blur', 'brighten', 'colorize', 'crop', 'detect_horizontal_lines', 'detect_vertical_lines', 'edge_detection', 'edge_one', 'emboss', 'filter', 'fliph', 'flipv', 'gaussian_blur',  'horizontal_strips', 'identity', 'inc_brightness', 'laplace',  'noise_reduction', 'offset', 'offset_blue', 'offset_green', 'offset_red', 'prewitt_horizontal', 'primary', 'resize', 'rotate180', 'rotate270', 'rotate90', 'save', 'save_bytes', 'sharpen', 'sobel_horizontal', 'sobel_vertical', 'solarize', 'thumbnail', 'tint', 'unsharpen', 'vertical_strips']
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