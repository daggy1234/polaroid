import typing
from PIL import Image as PillowImage
from PIL import ImageOps
from wand.image import Image as WandImage
from polaroid import Image as PolaroidImage
import matplotlib.pyplot as plt
import functools
import time


def timer(function):
    @functools.wraps(function)
    def wrapper(*args, **kwargs) -> float:
        time_list = []
        for i in range(10):
            start = time.perf_counter()
            function(*args, **kwargs)
            end = time.perf_counter()
            time_list.append(end - start)
            i += 0
        return sum(time_list) / len(time_list) * 1000

    return wrapper

def timer_return(function):
    @functools.wraps(function)
    def wrapper(*args, **kwargs) -> float:
        time_list = []
        for i in range(10):
            start = time.perf_counter()
            out_v = function(*args, **kwargs)
            end = time.perf_counter()
            time_list.append(end - start)
            i += 0
        return sum(time_list) / len(time_list) * 1000 , out_v

    return wrapper

@timer_return
def pil_encode(path: str) -> PillowImage:
    return PillowImage.open(path)


@timer_return
def polaroid_encode(path: str) -> PolaroidImage:
    return PolaroidImage(path)


@timer_return
def wand_encode(path: str) -> WandImage:
    return WandImage(filename=path)


@timer
def polaroid_image_gray(im: PolaroidImage):
    im.grayscale()


@timer
def wand_image_gray(img: WandImage):
    img.transform_colorspace("gray")

@timer
def pillow_image_gray(img: WandImage):
    ImageOps.grayscale(img)


@timer
def pillow_image_resize(img: PillowImage):
    img.resize((100, 100), PillowImage.LANCZOS)

@timer
def wand_image_resize(img: WandImage):
    img.resize(width=100, height=100, filter=25)

@timer
def polaroid_image_resize(im: PolaroidImage):
    im.resize(100, 100 ,5)


@timer
def pil_decode(img: PillowImage,path_save: str) -> PillowImage:
    img.save(path_save)


@timer
def polaroid_decode(img: PolaroidImage ,path_save: str) -> PolaroidImage:
    img.save(path_save)


@timer
def wand_decode(img: WandImage,path_save: str) -> WandImage:
    img.save(filename=path_save)


def pil_vs_polaroid():
    print(100 * "=")
    print("Encoding Images from file")
    print(100 * "=")
    res_polaroid, img_polaroid = polaroid_encode("p1.jpg")
    res_pillow, img_pillow = pil_encode("p1.jpg")
    res_wand, img_wand = wand_encode("p1.jpg")
    encode_times = [img_polaroid, img_pillow, img_wand]
    print(f"Pillow   took: {res_pillow}ms")
    print(f"Polaroid took: {res_polaroid}ms")
    print(f"Wand     took: {res_wand}ms")
    print(100 * "=")
    print("Grayscale Image")
    print(100 * "=")
    res_gray_pil = pillow_image_gray(img_pillow)
    res_gray_polaroid = polaroid_image_gray(img_polaroid)
    res_gray_wand = wand_image_gray(img_wand)
    grayscale_times = [res_gray_polaroid, res_gray_pil, res_gray_wand]
    print(f"Pillow   took: {res_gray_pil}ms")
    print(f"Polaroid took: {res_gray_polaroid}ms")
    print(f"Wand     took: {res_gray_wand}ms")
    print(100 * "=")
    print("Lanczos image resize (100, 100)")
    print(100 * "=")
    res_resize_pil = pillow_image_resize(img_pillow)
    res_resize_polaroid = polaroid_image_resize(img_polaroid)
    res_resize_wand = wand_image_resize(img_wand)
    resize_times = [res_resize_polaroid, res_resize_pil, res_resize_wand]
    print(f"Pillow   took: {res_resize_pil}ms")
    print(f"Polaroid took: {res_resize_polaroid}ms")
    print(f"Wand     took: {res_resize_wand}ms")
    print(100 * "=")
    print("Write Image to file")
    print(100 * "=")
    res_decode_pil = pil_decode(img_pillow, "pil.png")
    res_decode_polaroid = polaroid_decode(img_polaroid, "pol.png")
    res_decode_wand = wand_decode(img_wand, "w.png")
    decode_times = [res_decode_polaroid, res_decode_pil, res_decode_wand]
    print(f"Pillow   took: {res_decode_pil}ms")
    print(f"Polaroid took: {res_decode_polaroid}ms")
    print(f"Wand     took: {res_decode_wand}ms")
    print(100 * "=")





if __name__ == "__main__":
    pil_vs_polaroid()
