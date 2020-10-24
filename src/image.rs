use image::{GenericImageView, ImageBuffer};
use photon_rs::helpers;
pub use photon_rs::PhotonImage;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::Python;

#[pyclass]
pub struct Image {
    pub img: PhotonImage,
}
impl Clone for Image {
    fn clone(&self) -> Self {
        self.dup()
    }
}

impl Image {
    pub(crate) fn dup(&self) -> Image {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        Image {
            img: PhotonImage::new(
                dyn_image.raw_pixels(),
                dyn_image.width(),
                dyn_image.height(),
            ),
        }
    }
}

#[pymethods]
impl Image {
    #[new]
    fn new(py: Python, obj: PyObject) -> PyResult<Self> {
        if let Ok(img_path) = obj.extract::<String>(py) {
            let img = image::open(&img_path).unwrap();

            let (width, height) = img.dimensions();

            // Convert the DynamicImage type to raw vec representing RGBA pixels (not RGB)
            let raw_pixels = img.to_rgba().to_vec();

            let photon_image = PhotonImage::new(raw_pixels, width, height);
            Ok(Image { img: photon_image })
        } else if let Ok(v) = obj.extract::<Vec<u8>>(py) {
            let img = image::load_from_memory(&*v).unwrap();
            let (width, height) = img.dimensions();
            let raw_pixels = img.to_rgba().to_vec();
            let photon_image = PhotonImage::new(raw_pixels, width, height);
            Ok(Image { img: photon_image })
        } else {
            Err(PyTypeError::new_err("Could not extract an image"))
        }
    }
    #[getter]
    fn format(&self) -> PyResult<&str> {
        let format = image::guess_format(self.img.get_raw_pixels().as_slice());
        let string: &str = match format.unwrap() {
            image::ImageFormat::PNG => "png",
            image::ImageFormat::JPEG => "jpeg",
            image::ImageFormat::GIF => "gif",
            image::ImageFormat::WEBP => "webp",
            image::ImageFormat::PNM => "pnm",
            image::ImageFormat::TIFF => "tiff",
            image::ImageFormat::TGA => "tga",
            image::ImageFormat::BMP => "bmp",
            image::ImageFormat::ICO => "ico",
            image::ImageFormat::HDR => "hdr",
        };
        Ok(string)
    }

    #[getter]
    fn size(&self) -> PyResult<(u32, u32)> {
        let height = self.img.get_height();
        let width = self.img.get_width();
        let tup = (width, height);
        Ok(tup)
    }

    #[getter]
    fn height(&self) -> PyResult<u32> {
        Ok(self.img.get_height())
    }

    #[getter]
    fn width(&self) -> PyResult<u32> {
        Ok(self.img.get_width())
    }

    #[getter]
    fn mode(&self) -> PyResult<&str> {
        let im = helpers::dyn_image_from_raw(&self.img);
        let mode = im.color();
        let str = match mode {
            image::ColorType::Gray(..) => "GRAY",
            image::ColorType::RGB(..) => "RGB",
            image::ColorType::Palette(..) => "PALETTE",
            image::ColorType::GrayA(..) => "GRAYA",
            image::ColorType::RGBA(..) => "RGBA",
            image::ColorType::BGR(..) => "BGR",
            image::ColorType::BGRA(..) => "BGRA",
        };
        Ok(str)
    }
    pub fn save(&mut self, img_path: &str) -> PyResult<()> {
        let img = &mut self.img;
        let raw_pixels = img.get_raw_pixels();
        let width = img.get_width();
        let height = img.get_height();

        let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
        let dynimage = image::DynamicImage::ImageRgba8(img_buffer);
        Ok(dynimage.save(img_path).unwrap())
    }

    pub fn save_bytes(&mut self) -> PyResult<&PyBytes> {
        unsafe {
            Python::with_gil(|_py| -> PyResult<&PyBytes> {
                let npy = Python::assume_gil_acquired();
                let temp = self.img.get_raw_pixels();
                let buf = temp.as_slice();
                Ok(PyBytes::new(npy, buf))
            })
        }
    }
}
