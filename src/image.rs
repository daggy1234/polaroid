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

pub fn extract_image(obj: PyObject) -> Image {
    Python::with_gil(|py| -> Image {
        match obj.extract::<Image>(py) {
            Ok(val) => val,
            Err(_e) => panic!("Unable to extract Image from object"),
        }
    })
}

impl Image {
    pub(crate) fn dup(&self) -> Image {
        let dyn_image = helpers::dyn_image_from_raw(&self.img);
        Image {
            img: PhotonImage::new(dyn_image.to_bytes(), dyn_image.width(), dyn_image.height()),
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
            let raw_pixels = img.to_rgba8().to_vec();

            let photon_image = PhotonImage::new(raw_pixels, width, height);
            Ok(Image { img: photon_image })
        } else if let Ok(v) = obj.extract::<Vec<u8>>(py) {
            let img = image::load_from_memory(&*v).unwrap();
            let (width, height) = img.dimensions();
            let raw_pixels = img.to_rgba8().to_vec();
            let photon_image = PhotonImage::new(raw_pixels, width, height);
            Ok(Image { img: photon_image })
        } else {
            Err(PyTypeError::new_err("Could not extract an image"))
        }
    }
    #[getter]
    fn format(&self) -> PyResult<&str> {
        let dyn_img = photon_rs::helpers::dyn_image_from_raw(&self.img);
        let slic = dyn_img.as_bytes();
        let format = image::guess_format(slic);
        let res = match format {
            Ok(f) => f,
            Err(_e) => return Ok("unknown"),
        };
        let string: &str = match res {
            image::ImageFormat::Png => "png",
            image::ImageFormat::Jpeg => "jpeg",
            image::ImageFormat::Gif => "gif",
            image::ImageFormat::WebP => "webp",
            image::ImageFormat::Pnm => "pnm",
            image::ImageFormat::Tiff => "tiff",
            image::ImageFormat::Tga => "tga",
            image::ImageFormat::Dds => "dds",
            image::ImageFormat::Bmp => "bmp",
            image::ImageFormat::Ico => "ico",
            image::ImageFormat::Hdr => "hdr",
            image::ImageFormat::Farbfeld => "farbfeld",
            image::ImageFormat::Avif => "avif",
            _ => "unkown",
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
            image::ColorType::L16 => "L16",
            image::ColorType::L8 => "L8",
            image::ColorType::La16 => "La16",
            image::ColorType::La8 => "La8",
            image::ColorType::Rgb8 => "Rgb8",
            image::ColorType::Rgb16 => "Rgb16",
            image::ColorType::Rgba8 => "Rgba8",
            image::ColorType::Rgba16 => "Rgba16",
            image::ColorType::Bgr8 => "BGR8",
            image::ColorType::Bgra8 => "BGRA8",
            _ => "Unknown",
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
        dynimage.save(img_path).unwrap();
        Ok(())
    }

    pub fn save_jpeg_bytes(&mut self, quality: u8) -> PyResult<&PyBytes> {
        let mut img = helpers::dyn_image_from_raw(&self.img);
        img = image::DynamicImage::ImageRgba8(img.to_rgba8());
        let outf = image::ImageOutputFormat::Jpeg(quality);
        let mut buffer = vec![];
        match img.write_to(&mut buffer, outf) {
            Ok(..) => ..,
            Err(e) => panic!("Error: {}", e),
        };
        unsafe {
            Python::with_gil(|_py| -> PyResult<&PyBytes> {
                let npy = Python::assume_gil_acquired();
                let buf = buffer.as_slice();
                Ok(PyBytes::new(npy, buf))
            })
        }
    }

    #[args(py_args = "*", image_format = "\"guess\"")]
    pub fn save_bytes(&mut self, image_format: &str) -> PyResult<&PyBytes> {
        let mut img = helpers::dyn_image_from_raw(&self.img);
        let buf = img.to_bytes();
        let outf;
        if image_format == "guess" {
            match image::guess_format(buf.as_slice()) {
                Ok(f) => outf = image::ImageOutputFormat::from(f),
                Err(_e) => {
                    outf = image::ImageOutputFormat::Png;
                }
            }
        } else {
            outf = match image_format {
                "png" => image::ImageOutputFormat::Png,
                "jpeg" => {
                    panic!("Jpeg cannot be saved normally, please use the save_jpeg_bytes option")
                }
                "ico" => image::ImageOutputFormat::Ico,
                "bmp" => image::ImageOutputFormat::Bmp,
                "tga" => image::ImageOutputFormat::Tga,
                "farbfeld" => image::ImageOutputFormat::Farbfeld,
                _ => panic!("No valid Image format provided"),
            }
        }

        img = image::DynamicImage::ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        match img.write_to(&mut buffer, outf) {
            Ok(..) => ..,
            Err(e) => panic!("Error: {}", e),
        };
        unsafe {
            Python::with_gil(|_py| -> PyResult<&PyBytes> {
                let npy = Python::assume_gil_acquired();
                let buf = buffer.as_slice();
                Ok(PyBytes::new(npy, buf))
            })
        }
    }

    pub fn save_base_64(&mut self) -> PyResult<String> {
        let data = self.img.get_base64();
        Ok(data)
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for Image {
    fn __repr__(&self) -> PyResult<String> {
        let height = &self.height().unwrap();
        let width = &self.width().unwrap();
        let format = &self.format().unwrap();
        let mode = &self.mode().unwrap();
        Ok(format!(
            "<polaroid.Image height={} width={} format='{}' mode='{}'>",
            height, width, format, mode
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        let height = &self.height().unwrap();
        let width = &self.width().unwrap();
        let format = &self.format().unwrap();
        let mode = &self.mode().unwrap();
        Ok(format!(
            "<polaroid.Image height={} width={} format='{}' mode='{}'>",
            height, width, format, mode
        ))
    }
}
