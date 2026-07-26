//C. Wyatt Polasek + Zach Breene
//Rpeg Codec

use crate::rgb_module;
use crate::image_module;
use crate::codeword_module;

use csc411_image::{Read, Write, RgbImage};
use csc411_rpegio::{input_rpeg_data, output_rpeg_data};
use std::error::Error;

/// Compresses an RGB image to RPEG format.
///
/// # Arguments
///
/// * `filename` - An option containing the path to the PPM image file to be compressed.
///
/// # Returns
///
/// A `Result` which is `Ok` if the compression succeeded, or an `Error` if it failed.
pub fn compress(filename: Option<&str>) -> Result<(), Box<dyn Error>> {
    //Step 1: Read PPM Image
    let mut rgb_image = RgbImage::read(filename)?;

    // Trim the image if width or height is not even
    let (new_width, new_height) = (
        if rgb_image.width % 2 == 1 { rgb_image.width - 1 } else { rgb_image.width },
        if rgb_image.height % 2 == 1 { rgb_image.height - 1 } else { rgb_image.height },
    );

    if new_width != rgb_image.width || new_height != rgb_image.height {
        // Create a new vector to hold the trimmed pixels
        let mut trimmed_pixels = Vec::with_capacity((new_width * new_height) as usize);
    
        // Copy the pixels, excluding the last row and column if necessary
        for y in 0..new_height {
            for x in 0..new_width {
                let index = (y * rgb_image.width + x) as usize;
                // Clone the pixel to avoid moving it
                trimmed_pixels.push(rgb_image.pixels[index].clone());
            }
        }
    
        // Create a new RgbImage with the trimmed dimensions and pixels
        rgb_image = RgbImage {
            pixels: trimmed_pixels,
            width: new_width,
            height: new_height,
            denominator: rgb_image.denominator,
        };
    }

    //Step 2: Convert RGB to Component Video
    let component_video = rgb_module::convert_to_component_video(&rgb_image)?;

    //Step 3: Process Image Blocks and Apply DCT
    let dct_blocks = image_module::process_blocks_and_dct(&component_video, rgb_image.width as usize)?;

    //Step 4: Quantize and Pack Data
    let codewords = codeword_module::quantize_and_pack(&dct_blocks)?;

    //Convert codewords from Vec<u32> to Vec<[u8; 4]>
    let codewords_bytes: Vec<[u8; 4]> = codewords.iter()
    .map(|&word| word.to_be_bytes()) //Convert each u32 to big-endian bytes
    .collect();

    //Step 5: Write Compressed Data
    output_rpeg_data(&codewords_bytes, rgb_image.width as usize, rgb_image.height as usize)?;

    Ok(())
}

/// Decompresses an RPEG format image back into an RGB image.
///
/// # Arguments
///
/// * `filename` - An option containing the path to the RPEG compressed file to be decompressed.
///
/// # Returns
///
/// A `Result` which is `Ok` if the decompression succeeded, or an `Error` if it failed.
pub fn decompress(filename: Option<&str>) -> Result<(), Box<dyn Error>> {
    //Step 1: Read Compressed Data
    let (compressed_data, width, height) = input_rpeg_data(filename)?;

    //Step 2: Unpack and Dequantize Data
    let dct_blocks = codeword_module::unpack_and_dequantize(&compressed_data)?;

    //Step 3: Apply Inverse DCT
    let component_video = image_module::inverse_dct(&dct_blocks)?;

    //Step 4: Convert Component Video to RGB
    let rgb_image = rgb_module::component_video_to_rgb(&component_video, width, height)?;

    //Step 5: Write Decompressed Image
    rgb_image.write(None)?;

    Ok(())
}