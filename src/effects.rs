use crate::image::Image;
use image::imageops;
use image::{DynamicImage::ImageRgba8, GenericImageView};
use photon_rs::effects;
use photon_rs::helpers;
use photon_rs::PhotonImage;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pymethods]
impl Image {
    /// Adjust the contrast of an image by a factor.
    ///
    /// Example
    ///
    ///     ```no_run
    ///     img.colorize()
    ///     ```
    fn colorize(&mut self) -> PyResult<()> {
        effects::colorize(&mut self.img);
        Ok(())
    }

    /// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int`
    ///     the number of strips
    ///
    /// Example
    /// -------
    /// .. code-block:: python3
    ///     :linenos:
    ///     img.horizontal_strips(8)
    ///  
    #[text_signature = "(num: int)"]
    fn horizontal_strips(&mut self, num: u8) -> PyResult<()> {
        effects::horizontal_strips(&mut self.img, num);
        Ok(())
    }

    /// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int`
    ///     the number of strips
    ///
    /// Example
    /// -------
    /// .. code-block:: python3
    ///     :linenos:
    ///     img.vertical_strips(8)
    ///
    #[text_signature = "(num: int)"]
    fn vertical_strips(&mut self, num: u8) -> PyResult<()> {
        effects::vertical_strips(&mut self.img, num);
        Ok(())
    }

    /// Increase the brightness of an image by a factor. Different algoritihim
    ///
    /// # Arguments
    ///
    /// * `brightness` - A u8 to add to the brightness.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.inc_brightness(10)
    /// ```
    fn inc_brightness(&mut self, num: u8) -> PyResult<()> {
        effects::inc_brightness(&mut self.img, num);
        Ok(())
    }
    /// Reduces an image to the primary colours.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.primary()
    /// ```
    fn primary(&mut self) -> PyResult<()> {
        effects::primary(&mut self.img);
        Ok(())
    }

    /// Adjust the contrast of an image by a factor.
    ///
    /// # Arguments
    ///
    /// * `num` - factor to adjust contrast by
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.adjust_contrast(30)
    /// ```
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

    /// Applies a solarizing effect to an image.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.solarize()
    /// ```
    fn solarize(&mut self) -> PyResult<()> {
        effects::solarize(&mut self.img);
        Ok(())
    }

    /// Tint an image by adding an offset to averaged RGB channel values.
    ///
    /// # Arguments
    ///
    /// * `red_offset` - factor to increment r channel by
    /// * `green_offset` - factor to increment g channel by
    /// * `blue_offset` - factor to increment b channel by
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.tint(10, 20, 15)
    /// ```
    fn tint(&mut self, red_offset: u32, green_offset: u32, blue_offset: u32) -> PyResult<()> {
        effects::tint(&mut self.img, red_offset, green_offset, blue_offset);
        Ok(())
    }

    /// Adds an offset to the image by a certain number of pixels. This creates an RGB shift effect.
    ///
    /// Arguments:
    ///
    /// * `channel_index` - index of channel to increment. 0 for R, 1 for G and 2 for B.
    /// * `offset` - offset added to pixels in the image.
    ///
    /// Example:
    ///
    /// ```no_run
    /// img.offset(0, 30)
    /// ```
    fn offset(&mut self, channel_index: usize, offset: u32) -> PyResult<()> {
        effects::offset(&mut self.img, channel_index, offset);
        Ok(())
    }

    /// Adds an offset to the blue channel by a certain number of pixels.
    ///
    /// # Arguments
    ///
    /// * `offset` - offset added to pixels in the image.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.offset_blue(0, 30)
    /// ```
    fn offset_blue(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_blue(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the red channel by a certain number of pixels.
    ///
    /// # Arguments
    ///
    /// * `offset` - offset added to pixels in the image.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.offset_red(30)
    /// ```
    fn offset_red(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_red(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the green channel by a certain number of pixels.
    ///
    /// # Arguments
    ///
    /// * `offset` - offset added to pixels in the image.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.offset_green(30)
    /// ```
    fn offset_green(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_green(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the image by a certain number of pixels. This creates an RGB shift effect.
    ///
    /// # Arguments
    ///
    /// * `sigma` - is the amount to blur the image by.
    /// * `treshold` - treshold for the minimal brightness change that will be sharpened
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.unsharpen(8, 200)
    /// ```
    fn unsharpen(&mut self, sigma: f32, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::unsharpen(&img, sigma, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }

    /// Brighten the supplied imag
    ///
    /// # Arguments
    ///
    /// * `treshold` - is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.brighten(0, 200)
    /// ```
    fn brighten(&mut self, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::brighten(&img, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }

    /// Invert each pixel within the supplied image
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.invert()
    /// ```
    ///
    fn invert(&mut self) -> PyResult<()> {
        let mut img = helpers::dyn_image_from_raw(&self.img);
        imageops::invert(&mut img);
        self.img = PhotonImage::new(img.to_bytes(), img.width(), img.height());
        Ok(())
    }

    /// Turn an image into an oil painting
    ///
    /// # Arguments
    ///
    /// * `radius` - Radius of each paint particle.
    /// * `intensity` - How artsy an Image should be
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.oil(4, 55.0)
    /// ```
    fn oil(&mut self, radius: i32, intensity: f64) -> PyResult<()> {
        effects::oil(&mut self.img, radius, intensity);
        Ok(())
    }

    /// Turn an image into an frosted glass see through
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.frosted_glass()
    /// ```
    fn frosted_glass(&mut self) -> PyResult<()> {
        effects::frosted_glass(&mut self.img);
        Ok(())
    }

    /// Halftoning effect
    ///
    /// # Example
    ///
    /// ```no_run
    /// img.halftone()
    /// ```
    fn halftone(&mut self) -> PyResult<()> {
        effects::halftone(&mut self.img);
        Ok(())
    }

    /// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Specify a color as well
    ///
    /// # Arguments
    ///
    /// * `num` - the number of strips
    /// * `rgb` - the rgb color
    ///
    /// # Example
    ///
    /// ```no_run
    /// from photon import Rgb
    /// red = Rgb(255, 0 , 0)
    /// img.vertical_strips_coloured(red)
    /// ```
    fn vertical_strips_coloured(&mut self, num: u8) -> PyResult<()> {
        effects::vertical_strips(&mut self.img, num);
        Ok(())
    }

    /// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect. Specify a color as well
    ///
    /// # Arguments
    ///
    /// * `num` - the number of strips
    /// * `rgb` - the rgb color
    ///
    /// # Example
    ///
    /// ```no_run
    /// from photon import Rgb
    /// blue = Rgb(0, 0 , 255)
    /// img.horizontal_strips_coloured(blue)
    /// ```
    fn horizontal_strips_coloured(&mut self, num: u8) -> PyResult<()> {
        effects::inc_brightness(&mut self.img, num);
        Ok(())
    }
}
