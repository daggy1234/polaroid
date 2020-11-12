mod channels;
mod conv;
mod effects;
mod filters;
mod gif;
mod image;
mod monochrome;
mod multiple;
mod noise;
mod rgb;
mod transformations;

use pyo3::prelude::*;

#[pymodule]
fn polaroid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<image::Image>()?;
    m.add_class::<gif::Gif>()?;
    m.add_class::<rgb::Rgb>()?;
    Ok(())
}
