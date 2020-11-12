use crate::image::Image;
use photon_rs::filters;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
#[pymethods]
impl Image {
    #[allow(clippy::unit_arg)]
    fn filter(&mut self, filter: &str) -> PyResult<()> {
        let im = &mut self.img;
        match filter {
            "cali" => Ok(filters::cali(im)),
            "dramatic" => Ok(filters::dramatic(im)),
            "firenze" => Ok(filters::firenze(im)),
            "golden" => Ok(filters::golden(im)),
            "lix" => Ok(filters::lix(im)),
            "lofi" => Ok(filters::lofi(im)),
            "neue" => Ok(filters::neue(im)),
            "obsidian" => Ok(filters::obsidian(im)),
            "pastel_pink" => Ok(filters::pastel_pink(im)),
            "ryo" => Ok(filters::ryo(im)),
            "oceanic" => Ok(filters::filter(im, "oceanic")),
            "islands" => Ok(filters::filter(im, "islands")),
            "marine" => Ok(filters::filter(im, "marine")),
            "seagreen" => Ok(filters::filter(im, "seagreen")),
            "flagblue" => Ok(filters::filter(im, "flagblue")),
            "liquid" => Ok(filters::filter(im, "liquid")),
            "diamante" => Ok(filters::filter(im, "diamante")),
            "radio" => Ok(filters::filter(im, "radio")),
            "twenties" => Ok(filters::filter(im, "twenties")),
            "rosetint" => Ok(filters::filter(im, "rosetint")),
            "mauve" => Ok(filters::filter(im, "mauve")),
            "bluechrome" => Ok(filters::filter(im, "bluechrome")),
            "vintage" => Ok(filters::filter(im, "vintage")),
            "purfume" => Ok(filters::filter(im, "perfume")),
            "serenity" => Ok(filters::filter(im, "serenity")),
            _ => Err(PyValueError::new_err("Invalid Filter")),
        }
    }
}
