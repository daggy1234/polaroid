use crate::image::Image;
use image::gif::{Decoder, Encoder};
use image::AnimationDecoder;
use image::GenericImageView;
use photon_rs::PhotonImage;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use std::fs::File;

#[pyclass]
pub struct Gif {
    frames: Vec<Image>,
}
#[pymethods]
impl Gif {
    #[new]
    fn open(_py: Python, path: &str) -> PyResult<Self> {
        let file = File::open(path)?;
        let decoder = Decoder::new(file).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding gif");
        let mut f_vec = Vec::new();
        for frame in frames {
            let buffer = frame.into_buffer();
            let base = image::DynamicImage::ImageRgba8(buffer);
            let photon_image = PhotonImage::new(base.raw_pixels(), base.width(), base.height());
            f_vec.push(Image { img: photon_image })
        }
        Ok(Gif { frames: f_vec })
    }

    fn iterator(&mut self) -> PyResult<Vec<Image>> {
        Ok(self.frames.iter().map(|img| img.dup()).collect())
    }

    #[staticmethod]
    fn save_bytes(ts: &PyList) -> PyResult<&PyBytes> {
        let mut byt: Vec<u8> = Vec::new();

        {
            let mut encoder = Encoder::new(&mut byt);
            let vec: Vec<image::Frame> = ts
                .iter()
                .map(|t| -> image::Frame {
                    if let Ok(obj) = t.extract::<Image>() {
                        println!("Some data");
                        let img = &obj.img;
                        let raw_pixels = img.get_raw_pixels();
                        let width = img.get_width();
                        let height = img.get_height();

                        let buffs = match image::RgbaImage::from_raw(width, height, raw_pixels) {
                            Some(b) => Ok(b),
                            None => Err(PyRuntimeError::new_err("Broke")),
                        };
                        let frame = image::Frame::new(buffs.unwrap());
                        println!("frame");
                        return frame;
                    } else {
                        println!("Some");
                        panic!("Err")
                    };
                })
                .collect();

            println!("Data done");
            let _ret = encoder.encode_frames(vec).unwrap();
        }
        unsafe {
            Python::with_gil(|_py| -> PyResult<&PyBytes> {
                let npy = Python::assume_gil_acquired();
                let temp = byt;
                let byt = temp.as_slice();
                Ok(PyBytes::new(npy, byt))
            })
        }
    }

    #[staticmethod]
    fn save(path: &str, ts: &PyList) -> PyResult<()> {
        let file_out = File::create(path)?;
        let mut encoder = Encoder::new(file_out);
        println!("{}", ts.len());
        let vec: Vec<image::Frame> = ts
            .iter()
            .map(|t| -> image::Frame {
                if let Ok(obj) = t.extract::<Image>() {
                    println!("Some data");
                    let img = &obj.img;
                    let raw_pixels = img.get_raw_pixels();
                    let width = img.get_width();
                    let height = img.get_height();

                    let buffs = match image::RgbaImage::from_raw(width, height, raw_pixels) {
                        Some(b) => Ok(b),
                        None => Err(PyRuntimeError::new_err("Broke")),
                    };
                    let frame = image::Frame::new(buffs.unwrap());
                    println!("frame");
                    return frame;
                } else {
                    println!("Some");
                    panic!("Err")
                };
            })
            .collect();

        println!("Data done");
        Ok(encoder.encode_frames(vec).unwrap())
    }
}
