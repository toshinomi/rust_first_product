extern crate image;

use image::DynamicImage;
use std::io;
use rust_first_product::module::color_reversal;
use rust_first_product::module::color_reversal::GoImageProc;

fn main() {
    println!("image directory : ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    directory.pop();
    let input_image = directory.to_string() + "/input.jpg";
    let output_image = directory.to_string() + "/output.jpg";
    let mut img: DynamicImage = image::open(input_image).unwrap();
    
    println!("Image Processing Start!");

    color_reversal::ImageManager::new().go_image_proc(&mut img);
    //let mut color_reversal = color_reversal::ImageManager::new();
    //color_reversal.go_image_proc(&mut img);

    img.save(&output_image).unwrap();

    println!("Output Image : {}", output_image);

    println!("Image Processing End!");
}