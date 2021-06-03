use crate::image::Image;
use crate::rgb::extract_rgb;
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
    /// Examples
    /// --------
    /// .. code-block:: python3
    ///
    ///     img.colorize()
    ///
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
    /// Examples
    /// --------
    ///
    /// .. code-block:: python3
    ///
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
    ///
    /// .. code-block:: python3
    ///
    ///     img.vertical_strips(8)
    ///
    #[text_signature = "(num: int)"]
    fn vertical_strips(&mut self, num: u8) -> PyResult<()> {
        effects::vertical_strips(&mut self.img, num);
        Ok(())
    }

    /// Increase the brightness of an image by a factor. Different algoritihim
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int`
    ///     A u8 to add to the brightness.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.inc_brightness(10)
    ///
    fn inc_brightness(&mut self, num: u8) -> PyResult<()> {
        effects::inc_brightness(&mut self.img, num);
        Ok(())
    }
    /// Reduces an image to the primary colours.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.primary()
    ///
    fn primary(&mut self) -> PyResult<()> {
        effects::primary(&mut self.img);
        Ok(())
    }

    /// Adjust the contrast of an image by a factor.
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int`
    ///     factor to adjust contrast by
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.adjust_contrast()
    ///
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
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.solarize()
    ///
    fn solarize(&mut self) -> PyResult<()> {
        effects::solarize(&mut self.img);
        Ok(())
    }

    /// Tint an image by adding an offset to averaged RGB channel values.
    ///
    /// Parameters
    /// ----------
    /// red_offset: :class:`int`
    ///     factor to increment r channel by
    /// green_offset: :class:`int`
    ///     factor to increment g channel by
    /// blue_offset: :class:`int`
    ///     factor to increment b channel by
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.tint(10, 20 ,15)
    ///
    fn tint(&mut self, red_offset: u32, green_offset: u32, blue_offset: u32) -> PyResult<()> {
        effects::tint(&mut self.img, red_offset, green_offset, blue_offset);
        Ok(())
    }

    /// Adds an offset to the image by a certain number of pixels. This creates an RGB shift effect.
    ///
    /// Parameters
    /// ----------
    /// channel_index: :class:`int`
    ///     index of channel to increment. 0 for R, 1 for G and 2 for B.
    /// offset: :class:`int`
    ///     offset added to pixels in the image.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.offset(0, 30)
    ///
    fn offset(&mut self, channel_index: usize, offset: u32) -> PyResult<()> {
        effects::offset(&mut self.img, channel_index, offset);
        Ok(())
    }

    /// Adds an offset to the blue channel by a certain number of pixels.
    ///
    /// Parameters
    /// ----------
    /// offset: :class:`int`
    ///     offset added to pixels in the image.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.offset_blue(30)
    ///
    fn offset_blue(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_blue(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the red channel by a certain number of pixels.
    ///
    /// Parameters
    /// ----------
    /// offset: :class:`int`
    ///     offset added to pixels in the image.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.offset_red(30)
    ///
    fn offset_red(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_red(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the green channel by a certain number of pixels.
    ///
    /// Parameters
    /// ----------
    /// offset: :class:`int`
    ///     offset added to pixels in the image.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.offset_green(30)
    ///
    fn offset_green(&mut self, offset: u32) -> PyResult<()> {
        effects::offset_green(&mut self.img, offset);
        Ok(())
    }

    /// Adds an offset to the image by a certain number of pixels. This creates an RGB shift effect.
    ///
    /// Parameters
    /// ----------
    /// sigma: :class:`float`
    ///     the amount to blur the image by.
    /// treshold: :class:`int`
    ///     treshold for the minimal brightness change that will be sharpened
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.unsharpen(8, 200)
    ///
    fn unsharpen(&mut self, sigma: f32, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::unsharpen(&img, sigma, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }

    /// Brighten the supplied imag
    ///
    /// Parameters
    /// ----------
    /// treshold: :class:`int`
    ///     treshold for the minimal brightness change that will be sharpened
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.brighten(200)
    ///
    fn brighten(&mut self, treshold: i32) -> PyResult<()> {
        let img = helpers::dyn_image_from_raw(&self.img);
        let invert = ImageRgba8(imageops::brighten(&img, treshold));
        self.img = PhotonImage::new(invert.to_bytes(), invert.width(), invert.height());
        Ok(())
    }

    /// Invert each pixel within the supplied image
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.invert()
    ///
    fn invert(&mut self) -> PyResult<()> {
        let mut img = helpers::dyn_image_from_raw(&self.img);
        imageops::invert(&mut img);
        self.img = PhotonImage::new(img.to_bytes(), img.width(), img.height());
        Ok(())
    }

    /// Turn an image into an oil painting
    ///
    /// Parameters
    /// ----------
    /// radius: :class:`int`
    ///     Radius of each paint particle.
    /// intensity: :class:`float`
    ///     How artsy an Image should be
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.oil(4, 55.0)
    ///
    fn oil(&mut self, radius: i32, intensity: f64) -> PyResult<()> {
        effects::oil(&mut self.img, radius, intensity);
        Ok(())
    }

    /// Turn an image into an frosted glass see through
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.frosted_glass()
    ///
    fn frosted_glass(&mut self) -> PyResult<()> {
        effects::frosted_glass(&mut self.img);
        Ok(())
    }

    /// Halftoning effect
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     img.halftone()
    ///
    fn halftone(&mut self) -> PyResult<()> {
        effects::halftone(&mut self.img);
        Ok(())
    }

    /// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Specify a color as well
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int`
    ///     the number of strips
    /// rgb: :class:`Rgb`
    ///     rgb color for strips
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     from photon import Rgb
    ///     red = Rgb(255, 0 , 0)
    ///     img.vertical_strips_coloured(red)
    ///
    fn vertical_strips_coloured(&mut self, num: u8, rgb: PyObject) -> PyResult<()> {
        let rgb = extract_rgb(rgb);
        effects::color_horizontal_strips(&mut self.img, num, rgb.rgb);
        Ok(())
    }

    /// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect. Specify a color as well
    ///
    /// Parameters
    /// ----------
    /// num: :class:`int
    ///     the number of strips
    /// rgb: :class:`Rgb`
    ///     rgb color for strips
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: python3
    ///
    ///     from photon import Rgb
    ///     red = Rgb(0, 0 , 255)
    ///     img.vertical_strips_coloured(blue)
    ///
    fn horizontal_strips_coloured(&mut self, num: u8, rgb: PyObject) -> PyResult<()> {
        let rgb = extract_rgb(rgb);
        effects::color_horizontal_strips(&mut self.img, num, rgb.rgb);
        Ok(())
    }
}
