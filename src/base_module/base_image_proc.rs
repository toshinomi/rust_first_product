use image::DynamicImage;

pub trait GoImageProc {
    fn go_image_processing(&mut self, img: &mut DynamicImage);
}