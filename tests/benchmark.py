from PIL import Image as PillowImage
from PIL import ImageOps
from io import BytesIO
from wand.image import Image as WandImage
from polaroid import Image as PolaroidImage
import functools
import time
import os


def timer(function):
    @functools.wraps(function)
    def wrapper(*args, **kwargs) -> float:
        time_list = []
        for i in range(0, 100):
            start = time.perf_counter()
            function(*args, **kwargs)
            end = time.perf_counter()
            time_list.append(end - start)
            i += 0
        return round((sum(time_list) / len(time_list)) * 1000, 2)

    return wrapper


@timer
def pil_gray(byt: bytes):
    im = PillowImage.open(BytesIO(byt))
    ImageOps.grayscale(im)
    io = BytesIO()
    im.save(io, format=im.format)
    return im


@timer
def polaroid_gray(byt: bytes):
    im = PolaroidImage(byt)
    im.grayscale()
    return im.save_bytes()


@timer
def wand_gray(byt: bytes):
    with WandImage(blob=byt) as img:
        img.transform_colorspace("gray")
        return img.make_blob()


@timer
def polaroid_image_save(byt: bytes):
    im = PolaroidImage(byt)
    im.solarize()
    im.save("solarize.png")
    return "solarize.png"


@timer
def wand_image_save(byt: bytes):
    with WandImage(blob=byt) as img:
        img.solarize()
        img.save(filename="solarize.png")
        return "solarize.png"


@timer
def pil_image_save(byt: bytes):
    im = PillowImage.open(BytesIO(byt)).convert('RGB')
    ImageOps.solarize(im)
    im.save("solarize.png")
    return "solarize.png"


@timer
def io_read_pillow(path: str) -> bytes:
    im = PillowImage.open(path)
    im.resize((100, 100), PillowImage.NEAREST)
    io = BytesIO()
    im.save(io, format=im.format)
    return io.read()


@timer
def io_read_polaroid(path: str) -> bytes:
    im = PolaroidImage(path)
    im.resize(100, 100, 1)
    return im.save_bytes()


@timer
def io_read_wand(path: str) -> bytes:
    with WandImage(filename=path) as file:
        file.resize(100, 100)
        return file.make_blob()


def pil_vs_polaroid():
    byt = open("p1.jpg", "rb").read()
    res_polaroid = polaroid_gray(byt)
    res_pillow = pil_gray(byt)
    res_wand = wand_gray(byt)
    print(f"Pillow   took: {res_pillow}")
    print(f"Polaroid took: {res_polaroid}")
    print(f"Wand     took: {res_wand}")


def pil_vs_polaroid_save():
    byt = open("blue.png", "rb").read()
    res_polaroid = polaroid_image_save(byt)
    res_pillow = pil_image_save(byt)
    res_wand = wand_image_save(byt)
    os.remove("solarize.png")
    print(f"Pillow   took: {res_pillow}")
    print(f"Polaroid took: {res_polaroid}")
    print(f"Wand     took: {res_wand}")


def pil_vs_polaroid_io():
    res_polaroid = io_read_polaroid("p1.jpg")
    res_pillow = io_read_pillow("p1.jpg")
    res_wand = io_read_wand("p1.jpg")
    print(f"Pillow   took: {res_pillow}")
    print(f"Polaroid took: {res_polaroid}")
    print(f"Wand     took: {res_wand}")


if __name__ == "__main__":
    print("All values are in ms and consist of the average of 10 runs")
    print(100 * "=")
    print("Grayscale Encode and Decode: reading and writing to bytes while coverting image to grayscale")
    print(100 * "=")
    pil_vs_polaroid()
    print(100 * "=")
    print("Saving Images to File: using read image bytes and solarizing image and saving to file")
    print(100 * "=")
    pil_vs_polaroid_save()
    print(100 * "=")
    print("Reading Images from File and returning Bytes: Read Image from file resize to 100,100 and saving")
    print(100 * "=")
    pil_vs_polaroid_io()
