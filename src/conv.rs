use crate::image::Image;
use image::GenericImageView;
use imageproc::hog::*;
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

    fn hog(
        &mut self,
        orientations: usize,
        cell_size: usize,
        block_size: usize,
        block_stride: usize,
        star_side: u32,
        signed: bool,
    ) -> PyResult<()> {
        let opts = HogOptions {
            orientations,
            signed,
            cell_side: cell_size,
            block_side: block_size,
            block_stride,
        };
        let dyn_img = helpers::dyn_image_from_raw(&self.img).to_luma8();
        let (width, height) = dyn_img.dimensions();
        let spec = HogSpec::from_options(width, height, opts).unwrap();
        let mut hist = cell_histograms(&dyn_img, spec);
        let out = render_hist_grid(star_side, &hist.view_mut(), signed);
        let final_img =
            image::DynamicImage::ImageRgba8(image::DynamicImage::ImageLuma8(out).to_rgba8());
        self.img = PhotonImage::new(final_img.to_bytes(), final_img.width(), final_img.height());
        Ok(())
    }
}
