use crate::image::Image;
use image::imageops;
use image::{GenericImageView, ImageRgba8};
use photon_rs::conv;
use photon_rs::helpers;
use photon_rs::PhotonImage;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn box_blur(&mut self) -> PyResult<()> {
        conv::box_blur(&mut self.img);
        Ok(())
    }
    fn detect_horizontal_lines(&mut self) -> PyResult<()> {
        conv::detect_horizontal_lines(&mut self.img);
        Ok(())
    }
    fn detect_vertical_lines(&mut self) -> PyResult<()> {
        conv::detect_vertical_lines(&mut self.img);
        Ok(())
    }
    fn edge_detection(&mut self) -> PyResult<()> {
        conv::edge_detection(&mut self.img);
        Ok(())
    }
    fn edge_one(&mut self) -> PyResult<()> {
        conv::edge_one(&mut self.img);
        Ok(())
    }
    fn emboss(&mut self) -> PyResult<()> {
        conv::emboss(&mut self.img);
        Ok(())
    }
    fn gaussian_blur(&mut self, radius: i32) -> PyResult<()> {
        conv::gaussian_blur(&mut self.img, radius);
        Ok(())
    }
    fn identity(&mut self) -> PyResult<()> {
        conv::identity(&mut self.img);
        Ok(())
    }
    fn laplace(&mut self) -> PyResult<()> {
        conv::laplace(&mut self.img);
        Ok(())
    }
    fn noise_reduction(&mut self) -> PyResult<()> {
        conv::noise_reduction(&mut self.img);
        Ok(())
    }
    fn prewitt_horizontal(&mut self) -> PyResult<()> {
        conv::prewitt_horizontal(&mut self.img);
        Ok(())
    }
    fn sharpen(&mut self) -> PyResult<()> {
        conv::sharpen(&mut self.img);
        Ok(())
    }
    fn sobel_horizontal(&mut self) -> PyResult<()> {
        conv::sobel_horizontal(&mut self.img);
        Ok(())
    }
    fn sobel_vertical(&mut self) -> PyResult<()> {
        conv::sobel_vertical(&mut self.img);
        Ok(())
    }

    fn unsharpen(&mut self, sigma: f32, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::unsharpen(&img, sigma, treshold));
        self.img = PhotonImage::new(invert.raw_pixels(), invert.width(), invert.height());
        Ok(())
    }

    fn brighten(&mut self, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::brighten(&img, treshold));
        self.img = PhotonImage::new(invert.raw_pixels(), invert.width(), invert.height());
        Ok(())
    }
}
