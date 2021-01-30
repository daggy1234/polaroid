use crate::image::Image;
use crate::rgb::extract_rgb;
use image::GenericImageView;
use image::{DynamicImage, Luma, Rgb, Rgba};
use imageproc::map::map_colors;
use imageproc::pixelops::weighted_sum;
use photon_rs::{helpers::dyn_image_from_raw, PhotonImage};
use pyo3::prelude::*;

fn tint_gray(gray: Luma<u8>, color: Rgb<u8>) -> Rgb<u8> {
    let dist_from_mid = ((gray[0] as f32 - 128f32).abs()) / 255f32;
    let scale_factor = 1f32 - 4f32 * dist_from_mid.powi(2);
    weighted_sum(Rgb([gray[0]; 3]), color, 1.0, scale_factor)
}

fn tint_col(gray: Rgba<u8>, color: Rgb<u8>) -> Rgb<u8> {
    let dist_from_mid = ((gray[0] as f32 - 128f32).abs()) / 255f32;
    let scale_factor = 1f32 - 4f32 * dist_from_mid.powi(2);
    weighted_sum(Rgb([gray[0]; 3]), color, 1.0, scale_factor)
}

pub fn color_gradient(gray: Luma<u8>, low: Rgb<u8>, mid: Rgb<u8>, high: Rgb<u8>) -> Rgb<u8> {
    let fraction = gray[0] as f32 / 255f32;
    let (lower, upper, offset) = if fraction < 0.5 {
        (low, mid, 0.0)
    } else {
        (mid, high, 0.5)
    };
    let right_weight = 2.0 * (fraction - offset);
    let left_weight = 1.0 - right_weight;
    weighted_sum(lower, upper, left_weight, right_weight)
}

#[pymethods]
impl Image {
    fn color(&mut self, rgb: PyObject) -> PyResult<()> {
        let rgb = extract_rgb(rgb);
        let irgb = Rgb([rgb.rgb.get_red(), rgb.rgb.get_green(), rgb.rgb.get_blue()]);
        let img = dyn_image_from_raw(&self.img).to_luma8();
        let tinted = map_colors(&img, |pixel| tint_gray(pixel, irgb));
        let final_img = DynamicImage::ImageRgba8(DynamicImage::ImageRgb8(tinted).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }

    fn gradient(&mut self, low: PyObject, medium: PyObject, high: PyObject) -> PyResult<()> {
        let low = extract_rgb(low).to_image_rgb();
        let med = extract_rgb(medium).to_image_rgb();
        let hig = extract_rgb(high).to_image_rgb();
        let img = dyn_image_from_raw(&self.img).to_luma8();
        let tinted = map_colors(&img, |pixel| color_gradient(pixel, low, med, hig));
        let final_img = DynamicImage::ImageRgba8(DynamicImage::ImageRgb8(tinted).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }

    fn color_no_grayscale(&mut self, rgb: PyObject) -> PyResult<()> {
        let rgb = extract_rgb(rgb);
        let irgb = Rgb([rgb.rgb.get_red(), rgb.rgb.get_green(), rgb.rgb.get_blue()]);
        let img = dyn_image_from_raw(&self.img).to_rgba8();
        let tinted = map_colors(&img, |pixel| tint_col(pixel, irgb));
        let final_img = DynamicImage::ImageRgba8(DynamicImage::ImageRgb8(tinted).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }
}
