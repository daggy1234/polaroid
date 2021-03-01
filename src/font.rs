use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use rusttype;
use std::convert::TryFrom;
use std::include_bytes;

#[pyclass]
pub struct Font {
    font: rusttype::Font<'static>,
}

#[pymethods]
impl Font {
    #[new]
    pub fn new(py: Python, obj: PyObject) -> PyResult<Self> {
        if let Ok(img_path) = obj.extract::<String>(py) {
            let f = include_bytes!(&img_path);
            let font = rusttype::Font::try_from_bytes(f).unwrap();
            Ok(Font { font: font })
        } else if let Ok(v) = obj.extract::<Vec<u8>>(py) {
            let font = rusttype::Font::try_from_vec(v).unwrap();
            Ok(Font { font })
        } else {
            Err(PyTypeError::new_err("Could not extract font"))
        }
    }
}
