use crate::image::Image;
use crate::rgb::extract_rgb;
use photon_rs::channels;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    /// Alter a select channel by incrementing or decrementing its value by a constant.
    ///
    /// Parameters
    /// ----------
    /// index: :class:`int`
    ///     index of channel to increment. 0 for R, 1 for G and 2 for B.
    /// amount: :class:`int`
    ///     The amount to increment/decrement the channel’s value by for that pixel. A positive value will increment/decrement the channel’s value, a negative value will decrement the channel’s value.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.alter_channel(0, 10)
    ///
    fn alter_channel(&mut self, index: usize, amt: i16) -> PyResult<()> {
        channels::alter_channel(&mut self.img, index, amt);
        Ok(())
    }

    /// Increment all 3 channels’ values by adding an amt to each channel per pixel.
    ///
    /// Parameters
    /// ----------
    /// r_amt: :class:`int`
    ///     The amount to increment/decrement the Red channel by.
    /// g_amt: :class:`int`
    ///     The amount to increment/decrement the Green channel by.
    /// b_amt: :class:`int`
    ///     The amount to increment/decrement the Blue channel by.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.alter_channels(10, 20, 50)
    ///
    fn alter_channels(&mut self, r_amt: i16, b_amt: i16, g_amt: i16) -> PyResult<()> {
        channels::alter_channels(&mut self.img, r_amt, g_amt, b_amt);
        Ok(())
    }

    /// Increment or decrement every pixel’s Blue channel by a constant
    ///
    /// Parameters
    /// ----------
    /// amount: :class:`int`
    ///     The amount to increment or decrement the channel’s value by for that pixel.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.alter_blue_channel(10)
    ///
    fn alter_blue_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_blue_channel(&mut self.img, amt);
        Ok(())
    }

    /// Increment or decrement every pixel’s Blue channel by a constant
    ///
    /// Parameters
    /// ----------
    /// amount: :class:`int`
    ///     The amount to increment or decrement the channel’s value by for that pixel.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.alter_green_channel(10)
    ///
    fn alter_green_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_green_channel(&mut self.img, amt);
        Ok(())
    }

    /// Increment or decrement every pixel’s Red channel by a constant
    ///
    /// Parameters
    /// ----------
    /// amount: :class:`int`
    ///     The amount to increment or decrement the channel’s value by for that pixel.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.alter_red_channel(10)
    ///
    fn alter_red_channel(&mut self, amt: i16) -> PyResult<()> {
        channels::alter_red_channel(&mut self.img, amt);
        Ok(())
    }

    /// Remove the Blue channel’s influence in an image.
    ///
    /// Parameters
    /// ----------
    /// min_filter: :class:`int`
    ///     Only remove the channel if the current pixel’s channel value is less than this minimum filter.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.remove_blue_channel(50)
    ///
    fn remove_blue_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_blue_channel(&mut self.img, min_filter);
        Ok(())
    }

    /// Remove the Green channel’s influence in an image.
    ///
    /// Parameters
    /// ----------
    /// min_filter: :class:`int`
    ///     Only remove the channel if the current pixel’s channel value is less than this minimum filter.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.remove_green_channel(50)
    ///
    fn remove_green_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_green_channel(&mut self.img, min_filter);
        Ok(())
    }

    /// Remove the Red channel’s influence in an image.
    ///
    /// Parameters
    /// ----------
    /// min_filter: :class:`int`
    ///     Only remove the channel if the current pixel’s channel value is less than this minimum filter.
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///     img.remove_red_channel(50)
    ///
    fn remove_red_channel(&mut self, min_filter: u8) -> PyResult<()> {
        channels::remove_red_channel(&mut self.img, min_filter);
        Ok(())
    }

    /// Selectively desaturate pixel colours which are similar to the reference colour provided.
    ///
    /// Similarity between two colours is calculated via the CIE76 formula. Only desaturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm. For example, if a user wishes all pixels that are blue to be desaturated by 0.1, they can selectively specify only the blue pixels to be changed.
    ///
    /// Parameters
    /// ----------
    /// ref_color: :class:`Rgb`
    ///     The RGB value of the reference color (to be compared to)
    /// amt: :class:`float`
    ///     The amount to destarurate the color by
    ///
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
    ///    let rgb = polaroid.Rgb(20, 40, 60)
    ///    img.selective_desaturate(rgb, 0.1)
    ///
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

    fn channel_invert(&mut self) -> PyResult<()> {
        channels::invert(&mut self.img);
        Ok(())
    }
}
