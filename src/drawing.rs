use crate::image::{extract_image, Image};
use crate::rgb::extract_rgb;
use image::{DynamicImage, GenericImageView};
use imageproc::drawing;
use photon_rs::{helpers::dyn_image_from_raw, PhotonImage};
use pyo3::class::context::PyContextProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct ImageDraw {
    img: DynamicImage,
}

#[pymethods]
impl ImageDraw {
    #[new]
    fn new(_py: Python, obj: PyObject) -> PyResult<Self> {
        let img = extract_image(obj);
        let dyna = dyn_image_from_raw(&img.img);
        Ok(ImageDraw { img: dyna })
    }

    fn draw_cross(&mut self, pix: PyObject, x: i32, y: i32) -> PyResult<()> {
        let _rgb = extract_rgb(pix).to_image_rgb();
        let arr = [0u8, 0u8, 0u8, 255u8];
        let rgba = image::Rgba::<u8>::from(arr);
        drawing::draw_cross_mut(&mut self.img, rgba, x, y);
        Ok(())
    }

    fn draw_line_segment(&mut self, start: (f32, f32), end: (f32, f32)) -> PyResult<()> {
        let arr = [0u8, 0u8, 0u8, 255u8];
        let rgba = image::Rgba::<u8>::from(arr);
        drawing::draw_line_segment_mut(&mut self.img, start, end, rgba);
        Ok(())
    }

    fn draw(&mut self) -> PyResult<Image> {
        let final_img = &self.img;
        let pi = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(Image { img: pi })
    }
}

#[pyproto]
impl pyo3::class::context::PyContextProtocol for ImageDraw {
    fn __exit__(
        &mut self,
        _ty: Option<PyObject>,
        _value: Option<PyObject>,
        _traceback: Option<PyObject>,
    ) -> PyResult<Image> {
        let img = &self.img;
        let rgba = DynamicImage::ImageRgba8(img.to_rgba8());
        let pimg = PhotonImage::new(rgba.to_bytes(), rgba.width(), rgba.height());
        Ok(Image { img: pimg })
    }
}
