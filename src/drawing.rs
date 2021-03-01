use crate::rgb::extract_rgb;
use crate::{
    image::{extract_image, Image},
    rgb::Rgba,
};
use image::{DynamicImage, GenericImageView};
use imageproc::drawing;
use photon_rs::{helpers::dyn_image_from_raw, PhotonImage};
use pyo3::class::context::PyContextProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct ImageDraw {
    img: DynamicImage,
    phimg: Image,
}

#[pymethods]
impl ImageDraw {
    #[new]
    fn new(_py: Python, obj: Image) -> PyResult<Self> {
        let img = obj;
        let dyna = dyn_image_from_raw(&img.img);
        Ok(ImageDraw {
            img: dyna,
            phimg: img,
        })
    }

    fn cross(&mut self, coords: (i32, i32), color: Rgba) -> PyResult<()> {
        let rgba = color.to_image_rgba();
        drawing::draw_cross_mut(&mut self.img, rgba, coords.0, coords.1);
        Ok(())
    }

    fn line_segment(&mut self, start: (f32, f32), end: (f32, f32), color: Rgba) -> PyResult<()> {
        let rgba = color.to_image_rgba();
        drawing::draw_line_segment_mut(&mut self.img, start, end, rgba);
        Ok(())
    }

    fn circle_filled(&mut self, center: (i32, i32), radius: i32, color: Rgba) -> PyResult<()> {
        let rgba = color.to_image_rgba();
        drawing::draw_filled_circle_mut(&mut self.img, center, radius, rgba);
        Ok(())
    }

    fn circle_hollow(&mut self, center: (i32, i32), radius: i32, color: Rgba) -> PyResult<()> {
        let rgba = color.to_image_rgba();
        drawing::draw_hollow_circle_mut(&mut self.img, center, radius, rgba);
        Ok(())
    }

    // fn draw(&mut self) -> PyResult<()> {
    //     let final_img = &self.img;
    //     let pi = Image {
    //         img: PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height()),
    //     };
    //     self.phimg = pi;
    //     Ok(())
    // }

    fn draw(&mut self) -> PyResult<Image> {
        let final_img = &self.img;
        let pi = Image {
            img: PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height()),
        };
        Ok(pi)
    }
}

#[pyproto]
impl pyo3::class::context::PyContextProtocol for ImageDraw {
    fn __exit__(
        &mut self,
        _ty: Option<PyObject>,
        _value: Option<PyObject>,
        _traceback: Option<PyObject>,
    ) -> PyResult<()> {
        let img = &self.img;
        let rgba = DynamicImage::ImageRgba8(img.to_rgba8());
        let pimg = Image {
            img: PhotonImage::new(rgba.to_bytes(), rgba.width(), rgba.height()),
        };
        self.phimg = pimg;
        Ok(())
    }
}
