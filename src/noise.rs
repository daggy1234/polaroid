use crate::image::Image;
use photon_rs::noise;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn add_noise_rand(&mut self) -> PyResult<()> {
        let pimg = &self.img;
        let photon_img =
            photon_rs::PhotonImage::new(pimg.get_raw_pixels(), pimg.get_width(), pimg.get_height());
        let img = noise::add_noise_rand(photon_img);
        self.img = img;
        Ok(())
    }
    fn pink_noise(&mut self) -> PyResult<()> {
        noise::pink_noise(&mut self.img);
        Ok(())
    }
}
