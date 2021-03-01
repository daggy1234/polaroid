mod channels;
mod colorize;
mod conv;
// mod drawing;
mod effects;
mod filters;
//mod font;
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
    m.add_class::<rgb::Rgba>()?;
    // m.add_class::<drawing::ImageDraw>()?;
    Ok(())
}
