use crate::image::Image;
use photon_rs::effects;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn colorize(&mut self) -> PyResult<()> {
        Ok(effects::colorize(&mut self.img))
    }
    fn horizontal_strips(&mut self, num: u8) -> PyResult<()> {
        Ok(effects::horizontal_strips(&mut self.img, num))
    }
    fn vertical_strips(&mut self, num: u8) -> PyResult<()> {
        Ok(effects::vertical_strips(&mut self.img, num))
    }
    fn inc_brightness(&mut self, num: u8) -> PyResult<()> {
        Ok(effects::inc_brightness(&mut self.img, num))
    }
    fn primary(&mut self) -> PyResult<()> {
        Ok(effects::primary(&mut self.img))
    }

    fn adjust_contrast(&mut self, num: f32) -> PyResult<()> {
        if !(num <= -255.0 || num >= 255.0) {
            Ok(effects::adjust_contrast(&mut self.img, num))
        } else {
            Err(PyValueError::new_err(
                "Contrast must be a float between -255.0 and 255.0",
            ))
        }
    }
    fn solarize(&mut self) -> PyResult<()> {
        println!("Solared bitch");
        Ok(effects::solarize(&mut self.img))
    }
    fn tint(&mut self, red_offset: u32, green_offset: u32, blue_offset: u32) -> PyResult<()> {
        Ok(effects::tint(
            &mut self.img,
            red_offset,
            green_offset,
            blue_offset,
        ))
    }
    fn offset(&mut self, channel_index: usize, offset: u32) -> PyResult<()> {
        Ok(effects::offset(&mut self.img, channel_index, offset))
    }

    fn offset_blue(&mut self, offset: u32) -> PyResult<()> {
        Ok(effects::offset_blue(&mut self.img, offset))
    }
    fn offset_red(&mut self, offset: u32) -> PyResult<()> {
        Ok(effects::offset_red(&mut self.img, offset))
    }
    fn offset_green(&mut self, offset: u32) -> PyResult<()> {
        Ok(effects::offset_green(&mut self.img, offset))
    }
}
