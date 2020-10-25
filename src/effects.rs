use crate::image::Image;
use photon_rs::effects;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn colorize(&mut self) -> PyResult<()> {
        effects::colorize(&mut self.img);
        Ok(())
    }
    fn horizontal_strips(&mut self, num: u8) -> PyResult<()> {
        effects::horizontal_strips(&mut self.img, num);
        Ok(())
    }
    fn vertical_strips(&mut self, num: u8) -> PyResult<()> {
        effects::vertical_strips(&mut self.img, num);
        Ok(())
    }
    fn inc_brightness(&mut self, num: u8) -> PyResult<()> {
        effects::inc_brightness(&mut self.img, num);
        Ok(())
    }
    fn primary(&mut self) -> PyResult<()> {
        effects::primary(&mut self.img);
        Ok(())
    }

    fn adjust_contrast(&mut self, num: f32) -> PyResult<()> {
        if !(num <= -255.0 || num >= 255.0) {
            effects::adjust_contrast(&mut self.img, num);
            Ok(())
        } else {
            Err(PyValueError::new_err(
                "Contrast must be a float between -255.0 and 255.0",
            ))
        }
    }
    fn solarize(&mut self) -> PyResult<()> {
        effects::solarize(&mut self.img);
        Ok(())
    }
    fn tint(&mut self, red_offset: u32, green_offset: u32, blue_offset: u32) -> PyResult<()> {
        effects::tint(&mut self.img, red_offset, green_offset, blue_offset);
        Ok(())
    }
    fn offset(&mut self, channel_index: usize, offset: u32) -> PyResult<()> {
        effects::offset(&mut self.img, channel_index, offset);
        Ok(())
    }

    fn offset_blue(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_blue(&mut self.img, offset);
        Ok(())
    }
    fn offset_red(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_red(&mut self.img, offset);
        Ok(())
    }
    fn offset_green(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_green(&mut self.img, offset);
        Ok(())
    }
}
