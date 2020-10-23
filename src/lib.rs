mod conv;
mod effects;
mod filters;
mod gif;
mod image;
mod transformations;

use pyo3::prelude::*;

#[pymodule]
fn polaroid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<image::Image>()?;
    m.add_class::<gif::Gif>()?;
    Ok(())
}
