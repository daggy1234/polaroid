use crate::image::Image;
use image::imageops;
use image::{DynamicImage::ImageRgba8, GenericImageView};
use photon_rs::effects;
use photon_rs::helpers;
use photon_rs::PhotonImage;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn colorize(&mut self) -> PyResult<()> {
        effects::colorize(&mut self.img);
        Ok(())
    }
    fn horizontal_strips(&mut self, num: u8) -> PyResult<()> {
        effects::horizontal_strips(&mut self.img, num);
        Ok(())
    }
    fn vertical_strips(&mut self, num: u8) -> PyResult<()> {
        effects::vertical_strips(&mut self.img, num);
        Ok(())
    }
    fn inc_brightness(&mut self, num: u8) -> PyResult<()> {
        effects::inc_brightness(&mut self.img, num);
        Ok(())
    }
    fn primary(&mut self) -> PyResult<()> {
        effects::primary(&mut self.img);
        Ok(())
    }

    fn adjust_contrast(&mut self, num: f32) -> PyResult<()> {
        if !(num <= -255.0 || num >= 255.0) {
            effects::adjust_contrast(&mut self.img, num);
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Contrast must be a float between -255.0 and 255.0",
            ))
        }
    }
    fn solarize(&mut self) -> PyResult<()> {
        effects::solarize(&mut self.img);
        Ok(())
    }
    fn tint(&mut self, red_offset: u32, green_offset: u32, blue_offset: u32) -> PyResult<()> {
        effects::tint(&mut self.img, red_offset, green_offset, blue_offset);
        Ok(())
    }
    fn offset(&mut self, channel_index: usize, offset: u32) -> PyResult<()> {
        effects::offset(&mut self.img, channel_index, offset);
        Ok(())
    }

    fn offset_blue(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_blue(&mut self.img, offset);
        Ok(())
    }
    fn offset_red(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_red(&mut self.img, offset);
        Ok(())
    }
    fn offset_green(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_green(&mut self.img, offset);
        Ok(())
    }
    fn unsharpen(&mut self, sigma: f32, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::unsharpen(&img, sigma, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }

    fn brighten(&mut self, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::brighten(&img, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }
    fn invert(&mut self) -> PyResult<()> {
        let mut img = helpers::dyn_image_from_raw(&self.img);
        imageops::invert(&mut img);
        self.img = PhotonImage::new(img.to_bytes(), img.width(), img.height());
        Ok(())
    }

    fn oil(&mut self, radius: i32, intensity: f64) -> PyResult<()> {
        effects::oil(&mut self.img, radius, intensity);
        Ok(())
    }
}
