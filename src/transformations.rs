use crate::image::Image;
use image::GenericImageView;
use imageproc::seam_carving::*;
use photon_rs::helpers;
use photon_rs::transform;
use photon_rs::PhotonImage;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn crop(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> PyResult<()> {
        self.img = transform::crop(&mut self.img, x1, y1, x2, y2);
        Ok(())
    }

    fn fliph(&mut self) -> PyResult<()> {
        transform::fliph(&mut self.img);
        Ok(())
    }

    fn flipv(&mut self) -> PyResult<()> {
        transform::flipv(&mut self.img);
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32, filter: u8) -> PyResult<()> {
        let resample = match filter {
            1 => Ok(transform::SamplingFilter::Nearest),
            2 => Ok(transform::SamplingFilter::Triangle),
            3 => Ok(transform::SamplingFilter::CatmullRom),
            4 => Ok(transform::SamplingFilter::Gaussian),
            5 => Ok(transform::SamplingFilter::Lanczos3),
            _ => Err(PyValueError::new_err("Invalid Resampling FIlter Integer")),
        };
        match resample {
            Ok(filter) => {
                self.img = transform::resize(&self.img, width, height, filter);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
    fn rotate90(&mut self) -> PyResult<()> {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        let rotated_image = image::DynamicImage::ImageRgba8(image::imageops::rotate90(&dyn_image));
        self.img = PhotonImage::new(
            rotated_image.to_bytes(),
            rotated_image.width(),
            rotated_image.height(),
        );
        Ok(())
    }

    fn liquid_rescale(&mut self, width: u32, height: u32) -> PyResult<()> {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        let shrunk = dyn_image.to_rgb8();
        let p_a = shrink_width(&shrunk, width);
        let rotated = image::imageops::rotate90(&p_a);
        let h_a = shrink_width(&rotated, height);
        let out = image::imageops::rotate270(&h_a);
        let final_img =
            image::DynamicImage::ImageRgba8(image::DynamicImage::ImageRgb8(out).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }

    fn rotate180(&mut self) -> PyResult<()> {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        let rotated_image = image::DynamicImage::ImageRgba8(image::imageops::rotate180(&dyn_image));
        self.img = PhotonImage::new(
            rotated_image.to_bytes(),
            rotated_image.width(),
            rotated_image.height(),
        );
        Ok(())
    }

    fn rotate270(&mut self) -> PyResult<()> {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        let rotated_image = image::DynamicImage::ImageRgba8(image::imageops::rotate270(&dyn_image));
        self.img = PhotonImage::new(
            rotated_image.to_bytes(),
            rotated_image.width(),
            rotated_image.height(),
        );
        Ok(())
    }

    fn thumbnail(&mut self, width: u32, height: u32) -> PyResult<()> {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        let thumbnail =
            image::DynamicImage::ImageRgba8(image::imageops::thumbnail(&dyn_image, width, height));
        self.img = PhotonImage::new(thumbnail.to_bytes(), thumbnail.width(), thumbnail.height());
        Ok(())
    }
}
