use crate::image::Image;
use image;
use image::{GenericImageView, ImageRgba8};
use photon_rs::conv;
use photon_rs::helpers;
use photon_rs::PhotonImage;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn box_blur(&mut self) -> PyResult<()> {
        Ok(conv::box_blur(&mut self.img))
    }
    fn detect_horizontal_lines(&mut self) -> PyResult<()> {
        Ok(conv::detect_horizontal_lines(&mut self.img))
    }
    fn detect_vertical_lines(&mut self) -> PyResult<()> {
        Ok(conv::detect_vertical_lines(&mut self.img))
    }
    fn edge_detection(&mut self) -> PyResult<()> {
        Ok(conv::edge_detection(&mut self.img))
    }
    fn edge_one(&mut self) -> PyResult<()> {
        Ok(conv::edge_one(&mut self.img))
    }
    fn emboss(&mut self) -> PyResult<()> {
        Ok(conv::emboss(&mut self.img))
    }
    fn gaussian_blur(&mut self, radius: i32) -> PyResult<()> {
        Ok(conv::gaussian_blur(&mut self.img, radius))
    }
    fn identity(&mut self) -> PyResult<()> {
        Ok(conv::identity(&mut self.img))
    }
    fn laplace(&mut self) -> PyResult<()> {
        Ok(conv::laplace(&mut self.img))
    }
    fn noise_reduction(&mut self) -> PyResult<()> {
        Ok(conv::noise_reduction(&mut self.img))
    }
    fn prewitt_horizontal(&mut self) -> PyResult<()> {
        Ok(conv::prewitt_horizontal(&mut self.img))
    }
    fn sharpen(&mut self) -> PyResult<()> {
        Ok(conv::sharpen(&mut self.img))
    }
    fn sobel_horizontal(&mut self) -> PyResult<()> {
        Ok(conv::sobel_horizontal(&mut self.img))
    }
    fn sobel_vertical(&mut self) -> PyResult<()> {
        Ok(conv::sobel_vertical(&mut self.img))
    }

    fn unsharpen(&mut self, sigma: f32, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&mut self.img);
        let invert = ImageRgba8(image::imageops::unsharpen(&img, sigma, treshold));
        self.img = PhotonImage::new(invert.raw_pixels(), invert.width(), invert.height());
        Ok(())
    }

    fn brighten(&mut self, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&mut self.img);
        let invert = ImageRgba8(image::imageops::brighten(&img, treshold));
        self.img = PhotonImage::new(invert.raw_pixels(), invert.width(), invert.height());
        Ok(())
    }
}
