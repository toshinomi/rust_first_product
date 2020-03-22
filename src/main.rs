extern crate image;

use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use std::io;

fn main() {
    println!("image directory : ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    directory.pop();
    let input_image = directory.to_string() + "/input.jpg";
    let output_image = directory.to_string() + "/output.jpg";
    let mut img: DynamicImage = image::open(input_image).unwrap();
    
    println!("Image Processing Start!");

    color_reversal(&mut img);

    img.save(&output_image).unwrap();

    println!("Output Image : {}", output_image);

    println!("Image Processing End!");
}

fn color_reversal(img: &mut DynamicImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel: Rgba<u8> = img.get_pixel(x, y);
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