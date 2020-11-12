use crate::image::Image;
use crate::rgb::extract_rgb;
use photon_rs::channels;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    fn alter_channel(&mut self, index: usize, amt: i16) -> PyResult<()> {
        channels::alter_channel(&mut self.img, index, amt);
        Ok(())
    }
    fn alter_channels(&mut self, r_amt: i16, b_amt: i16, g_amt: i16) -> PyResult<()> {
        channels::alter_channels(&mut self.img, r_amt, g_amt, b_amt);
        Ok(())
    }
    fn alter_blue_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_blue_channel(&mut self.img, amt);
        Ok(())
    }
    fn alter_green_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_green_channel(&mut self.img, amt);
        Ok(())
    }
    fn alter_red_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_red_channel(&mut self.img, amt);
        Ok(())
    }
    fn remove_blue_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_blue_channel(&mut self.img, min_filter);
        Ok(())
    }
    fn remove_green_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_green_channel(&mut self.img, min_filter);
        Ok(())
    }
    fn remove_red_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_red_channel(&mut self.img, min_filter);
        Ok(())
    }

    fn selective_desaturate(&mut self, pixel: PyObject, amt: f32) -> PyResult<()> {
        let rgb = extract_rgb(pixel);
        channels::selective_desaturate(&mut self.img, rgb.rgb, amt);
        Ok(())
    }

    fn selective_hue_rotate(&mut self, pixel: PyObject, amt: f32) -> PyResult<()> {
        let rgb = extract_rgb(pixel);
        channels::selective_hue_rotate(&mut self.img, rgb.rgb, amt);
        Ok(())
    }
    fn selective_lighten(&mut self, pixel: PyObject, amt: f32) -> PyResult<()> {
        let rgb = extract_rgb(pixel);
        channels::selective_lighten(&mut self.img, rgb.rgb, amt);
        Ok(())
    }
    fn selective_saturate(&mut self, pixel: PyObject, amt: f32) -> PyResult<()> {
        let rgb = extract_rgb(pixel);
        channels::selective_saturate(&mut self.img, rgb.rgb, amt);
        Ok(())
    }
    fn swap_channels(&mut self, channel1: usize, channel2: usize) -> PyResult<()> {
        channels::swap_channels(&mut self.img, channel1, channel2);
        Ok(())
    }
}
