use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::base_module::common::Pixel;
use crate::base_module::common::PixelKind;
use crate::base_module::base_image_proc::GoImageProcessing;

pub struct ColorReversal {}

impl ColorReversal {
    pub fn new() -> ColorReversal {
        ColorReversal {}
    }
}

impl GoImageProcessing for ColorReversal {
    fn go_image_processing(&mut self, image: &mut DynamicImage) {
        let (width, height) = image.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = image.get_pixel(x, y);
                let mut pixel_data: Pixel<u8> = Pixel::new(
                    pixel[PixelKind::Red as usize],
                    pixel[PixelKind::Green as usize], 
                    pixel[PixelKind::Blue as usize],
                    pixel[PixelKind::Alpha as usize]
                );
                let red = pixel_data.get_red().clone();
                let green = pixel_data.get_green().clone();
                let blue = pixel_data.get_blue().clone();
                let alpha = pixel_data.get_alpha().clone();

                let new_color = [255 - red, 255 - green, 255 - blue, alpha];
                let pixel: Rgba<u8> = Rgba(new_color);

                image.put_pixel(x, y, pixel);
            }
        }
    }
}