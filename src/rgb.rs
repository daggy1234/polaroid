use pyo3::prelude::*;

pub fn extract_rgb(object: PyObject) -> Rgb {
    Python::with_gil(|py| -> Rgb {
        match object.extract::<Rgb>(py) {
            Ok(val) => val,
            Err(_e) => panic!("Unable to extract RGB from object"),
        }
    })
}

#[pyclass]
pub struct Rgb {
    pub rgb: photon_rs::Rgb,
}

impl Clone for Rgb {
    fn clone(&self) -> Self {
        let rgb = photon_rs::Rgb::new(
            self.rgb.get_red(),
            self.rgb.get_green(),
            self.rgb.get_blue(),
        );
        Rgb { rgb }
    }
}

impl Rgb {
    pub fn to_image_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([
            self.red().unwrap(),
            self.blue().unwrap(),
            self.green().unwrap(),
        ])
    }
}

#[pymethods]
impl Rgb {
    #[new]
    fn new(_py: Python, red: u8, green: u8, blue: u8) -> PyResult<Self> {
        let rgb = photon_rs::Rgb::new(red, green, blue);
        Ok(Rgb { rgb })
    }

    #[getter]
    fn red(&self) -> PyResult<u8> {
        Ok(self.rgb.get_red())
    }

    #[getter]
    fn green(&self) -> PyResult<u8> {
        Ok(self.rgb.get_green())
    }

    #[getter]
    fn blue(&self) -> PyResult<u8> {
        Ok(self.rgb.get_blue())
    }

    fn set_blue(&mut self, blue: u8) -> PyResult<()> {
        self.rgb.set_blue(blue);
        Ok(())
    }

    fn set_red(&mut self, red: u8) -> PyResult<()> {
        self.rgb.set_red(red);
        Ok(())
    }

    fn set_green(&mut self, green: u8) -> PyResult<()> {
        self.rgb.set_green(green);
        Ok(())
    }
}

#[pyclass]
pub struct Rgba {
    pub data: [u8; 4],
}

impl Rgba {
    pub fn to_image_rgba(&self) -> image::Rgba<u8> {
        image::Rgba::<u8>::from(self.data)
    }
}

impl Clone for Rgba {
    fn clone(&self) -> Self {
        let arr = self.data;
        Rgba { data: arr }
    }
}

#[pymethods]
impl Rgba {
    #[new]
    fn new(_py: Python, red: u8, green: u8, blue: u8, alpha: u8) -> PyResult<Self> {
        let arr: [u8; 4] = [red, green, blue, alpha];
        Ok(Rgba { data: arr })
    }

    #[getter]
    fn red(&self) -> PyResult<u8> {
        Ok(self.data[0])
    }

    #[getter]
    fn green(&self) -> PyResult<u8> {
        Ok(self.data[1])
    }

    #[getter]
    fn blue(&self) -> PyResult<u8> {
        Ok(self.data[2])
    }

    #[getter]
    fn alpha(&self) -> PyResult<u8> {
        Ok(self.data[3])
    }

    fn set_blue(&mut self, blue: u8) -> PyResult<()> {
        self.data[2] = blue;
        Ok(())
    }

    fn set_red(&mut self, red: u8) -> PyResult<()> {
        self.data[0] = red;
        Ok(())
    }

    fn set_green(&mut self, green: u8) -> PyResult<()> {
        self.data[1] = green;
        Ok(())
    }

    fn set_alpha(&mut self, alpha: u8) -> PyResult<()> {
        self.data[3] = alpha;
        Ok(())
    }
}
