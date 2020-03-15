extern crate image;

use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;

fn main() {
    let input_image = "/home/toshinomi/worksapce/rust/RustFirstProduct/image/input.jpg";
    let output_image = "/home/toshinomi/worksapce/rust/RustFirstProduct/image/output.jpg";
    let mut img: DynamicImage = image::open(input_image).unwrap();
    
    println!("Image Processing Start!");

    color_reversal(&mut img);

    img.save(output_image).unwrap();

    println!("Image Processing End!");
}

fn color_reversal(img: &mut DynamicImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let mut pixel: Rgba<u8> = img.get_pixel(x, y);
            let red = pixel[0];
            let green = pixel[1];
            let blue = pixel[2];
            let alpha = pixel[3];

            let new_color = [255 - red, 255 - green, 255 - blue, alpha];
            let pixel: Rgba<u8> = Rgba(new_color);

            img.put_pixel(x, y, pixel);
        }
    }
}