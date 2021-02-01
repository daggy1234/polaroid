use crate::image::Image;
use image::DynamicImage;
use image::GenericImageView;
use photon_rs::helpers::dyn_image_from_raw;
use photon_rs::monochrome;
use photon_rs::PhotonImage;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn b_grayscale(&mut self) -> PyResult<()> {
        monochrome::b_grayscale(&mut self.img);
        Ok(())
    }
    fn decompose_max(&mut self) -> PyResult<()> {
        monochrome::decompose_max(&mut self.img);
        Ok(())
    }
    fn decompose_min(&mut self) -> PyResult<()> {
        monochrome::decompose_min(&mut self.img);
        Ok(())
    }
    fn desaturate(&mut self) -> PyResult<()> {
        monochrome::desaturate(&mut self.img);
        Ok(())
    }
    fn g_grayscale(&mut self) -> PyResult<()> {
        monochrome::g_grayscale(&mut self.img);
        Ok(())
    }
    fn grayscale(&mut self) -> PyResult<()> {
        let img = dyn_image_from_raw(&self.img);
        let rgb = img.to_luma8();
        let final_img = DynamicImage::ImageRgba8(DynamicImage::ImageLuma8(rgb).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }
    fn grayscale_human_corrected(&mut self) -> PyResult<()> {
        monochrome::grayscale_human_corrected(&mut self.img);
        Ok(())
    }
    fn grayscale_shades(&mut self, num_of_shades: u8) -> PyResult<()> {
        monochrome::grayscale_shades(&mut self.img, num_of_shades);
        Ok(())
    }
    fn monochrome(&mut self, r_offset: u32, g_offset: u32, b_offset: u32) -> PyResult<()> {
        monochrome::monochrome(&mut self.img, r_offset, g_offset, b_offset);
        Ok(())
    }
    fn r_grayscale(&mut self) -> PyResult<()> {
        monochrome::r_grayscale(&mut self.img);
        Ok(())
    }
    fn sepia(&mut self) -> PyResult<()> {
        monochrome::sepia(&mut self.img);
        Ok(())
    }
    fn single_channel_grayscale(&mut self, channel: usize) -> PyResult<()> {
        monochrome::single_channel_grayscale(&mut self.img, channel);
        Ok(())
    }
    fn threshold(&mut self, treshold: u32) -> PyResult<()> {
        monochrome::threshold(&mut self.img, treshold);
        Ok(())
    }
}
