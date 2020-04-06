use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::module::common::Pixel;
use crate::module::common::PixelKind;

pub struct ImageManager {}

impl ImageManager {
    pub fn new() -> ImageManager {
        ImageManager {}
    }
}

pub trait GoImageProc {
    fn go_image_proc(&mut self, img: &mut DynamicImage) -> &mut ImageManager;
}

impl GoImageProc for ImageManager {
    fn go_image_proc(&mut self, img: &mut DynamicImage) -> &mut ImageManager {
        let (width, height) = img.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
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

                img.put_pixel(x, y, pixel);
            }
        }
        self
    }
}