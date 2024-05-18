use colored::Colorize;
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::path::Path;

pub fn generate_image() -> Result<(), Box<dyn Error>> {
    let img = image::open(&Path::new("snapshot.jpg"))?;

    // Resize the image
    let resized_img = img.resize(200, 133, image::imageops::FilterType::Nearest);

    // Convert the resized image to RGB and get raw pixel data
    let rgb_image = DynamicImage::ImageRgb8(resized_img.to_rgb8());
    let (width, height) = rgb_image.dimensions();

    let brightness_scale = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let mut brightness = Vec::new();

    let scale_len = brightness_scale.len();

    let pixels = rgb_image.pixels();

    for pixel in pixels {
        let rgb = pixel.2;
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];

        let avg_brightness = (r as u32 + g as u32 + b as u32) / 3;

        // Normalize the brightness to the range of the brightness scale
        let scale_index = (avg_brightness * (scale_len as u32 - 1) / 255) as usize;

        // Map the brightness to a character from the scale
        let char = brightness_scale.chars().nth(scale_index).unwrap();
        brightness.push(char);
    }

    for y in 0..height {
        for x in 0..width {
            let char = brightness[(y * width + x) as usize];
            print!("{}", format!("{char}{char}{char}")); // Print each character 3 times in green
        }
        println!();
    }

    Ok(())
}
