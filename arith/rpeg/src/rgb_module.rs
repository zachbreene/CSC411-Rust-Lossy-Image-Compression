//C. Wyatt Polasek + Zach Breene
//Rpeg Codec RGB Module

use csc411_image::{RgbImage, Rgb};
use std::error::Error;
use array2::Array2;

/// Converts integer RGB values of an image to floating-point RGB values.
///
/// # Arguments
///
/// * `rgb_image` - A reference to an `RgbImage` containing integer pixel values.
///
/// # Returns
///
/// A `Result` containing a vector of floating-point RGB values if successful, or an `Error` if not.
pub fn convert_to_float_rgb(rgb_image: &RgbImage) -> Result<Vec<(f64, f64, f64)>, Box<dyn Error>> {
    let max_val = rgb_image.denominator as f64;
    let float_pixels = rgb_image.pixels.iter()
        .map(|p| (
            p.red as f64 / max_val,
            p.green as f64 / max_val,
            p.blue as f64 / max_val
        ))
        .collect();
    Ok(float_pixels)
}

/// Converts floating-point RGB values to component video format (Y, Pb, Pr).
///
/// # Arguments
///
/// * `rgb_image` - A reference to an `RgbImage` containing floating-point RGB values.
///
/// # Returns
///
/// A `Result` containing a vector of component video format pixels if successful, or an `Error` if not.
pub fn convert_to_component_video(rgb_image: &RgbImage) -> Result<Vec<(f64, f32, f32)>, Box<dyn Error>> {
    let float_pixels = convert_to_float_rgb(rgb_image)?;
    let component_pixels = float_pixels.into_iter()
        .map(|(r, g, b)| rgb_to_ypbpr(r, g, b))
        .collect();
    Ok(component_pixels)
}

/// Converts a single RGB pixel to YPbPr format.
///
/// # Arguments
///
/// * `r` - Red component of the pixel.
/// * `g` - Green component of the pixel.
/// * `b` - Blue component of the pixel.
///
/// # Returns
///
/// A tuple representing the YPbPr components of the pixel.
fn rgb_to_ypbpr(r: f64, g: f64, b: f64) -> (f64, f32, f32) {
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let pb = -0.168736 * r - 0.331264 * g + 0.5 * b;
    let pr = 0.5 * r - 0.418688 * g - 0.081312 * b;
    (y, pb as f32, pr as f32)
}

/// Converts a vector of YPbPr pixel blocks to an RgbImage.
///
/// # Arguments
///
/// * `component_video` - A vector of blocks of YPbPr pixels.
/// * `width` - Width of the image in pixels.
/// * `height` - Height of the image in pixels.
///
/// # Returns
///
/// A `Result` containing the `RgbImage` if successful, or an `Error` if not.
pub fn component_video_to_rgb(component_video: &Vec<[(f64, f32, f32); 4]>, width: usize, height: usize) -> Result<RgbImage, Box<dyn Error>> {
    let mut image = Array2::new(width, height, Rgb { red: 0, green: 0, blue: 0 });
    
    //Enumerate over blocks with indices
    for (block_index, block) in component_video.iter().enumerate() {
        //Calculate the starting x and y coordinates
        let start_x = (block_index % (width / 2)) * 2;
        let start_y = (block_index / (width / 2)) * 2;
        
        //Enumerate over pixels within a block with their local block indices
        for (local_index, &(y, pb, pr)) in block.iter().enumerate() {
            let (r, g, b) = ypbpr_to_rgb(y, pb, pr);
            let rgb = Rgb {
                red: (r * 255.0).round() as u16,
                green: (g * 255.0).round() as u16,
                blue: (b * 255.0).round() as u16,
            };
            //Calculate the actual x and y coordinates
            let x = start_x + (local_index % 2);
            let y = start_y + (local_index / 2);
            image.set(x, y, rgb);
        }
    }
    
    // Convert Array2<Rgb> to RgbImage
    let pixels = image.data().clone();
    Ok(RgbImage {
        pixels,
        width: width as u32,
        height: height as u32,
        denominator: 255,
    })
}

/// Converts a single YPbPr pixel to an RGB pixel.
///
/// # Arguments
///
/// * `y` - Luminance component of the pixel.
/// * `pb` - Blue-difference chroma component of the pixel.
/// * `pr` - Red-difference chroma component of the pixel.
///
/// # Returns
///
/// A tuple representing the RGB components of the pixel.
fn ypbpr_to_rgb(y: f64, pb: f32, pr: f32) -> (f64, f64, f64) {
    let pb = pb as f64;
    let pr = pr as f64;
    let r = y + 1.402 * pr;
    let g = y - 0.344136 * pb - 0.714136 * pr;
    let b = y + 1.772 * pb;
    (r as f64, g as f64, b as f64)
}