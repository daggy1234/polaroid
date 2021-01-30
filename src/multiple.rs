use crate::image::{extract_image, Image};
use crate::rgb::extract_rgb;
use photon_rs::multiple;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn apply_gradient(&mut self) -> PyResult<()> {
        multiple::apply_gradient(&mut self.img);
        Ok(())
    }

    // fn create_gradient(self, width: u32, height: u32) -> PyResult<Image> {
    //     let img = multiple::create_gradient(width, height);
    //     Ok(Image { img })
    // }

    fn watermark(&mut self, obj: PyObject, x: u32, y: u32) -> PyResult<()> {
        let img = extract_image(obj);
        multiple::watermark(&mut self.img, &img.img, x, y);
        Ok(())
    }

    fn replace_backround(&mut self, obj: PyObject, rgb: PyObject) -> PyResult<()> {
        let rgb = extract_rgb(rgb);
        let img = extract_image(obj);
        multiple::replace_background(&mut self.img, &img.img, rgb.rgb);
        Ok(())
    }

    fn blend(&mut self, obj: PyObject, mode: &str) -> PyResult<()> {
        let img = extract_image(obj);
        if is_valid_blend(mode) {
            multiple::blend(&mut self.img, &img.img, mode);
            Ok(())
        } else {
            Err(PyValueError::new_err("Invalid Blend Mode Chosen"))
        }
    }
}

fn is_valid_blend(blend: &str) -> bool {
    let blend_vec = vec![
        "overlay",
        "over",
        "atop",
        "xor",
        "multiply",
        "burn",
        "soft_light",
        "hard_light",
        "difference",
        "lighten",
        "darken",
        "dodge",
        "plus",
        "exclusion",
    ];
    blend_vec.contains(&blend)
}
