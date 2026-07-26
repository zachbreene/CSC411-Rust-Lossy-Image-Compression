//C. Wyatt Polasek + Zach Breene
//Rpeg Codec Image Module

use std::error::Error;

type ComponentBlock = [(f64, f32, f32); 4]; //A block of 2x2 pixels
type DCTBlock = (f64, f64, f64, f64, f32, f32); //a, b, c, d, Pb, Pr

/// Processes the component video and applies Discrete Cosine Transform (DCT) to each block.
///
/// # Arguments
///
/// * `component_video` - A slice of YPbPr values representing the component video.
/// * `width` - The width of the video in pixels, must be even.
///
/// # Returns
///
/// A `Result` containing a vector of DCTBlocks if successful, or an `Error` if not.
pub fn process_blocks_and_dct(component_video: &[(f64, f32, f32)], width: usize) -> Result<Vec<DCTBlock>, Box<dyn Error>> {
    //Split the video into blocks
    let blocks = split_into_blocks(component_video, width)?;

    //Apply DCT to each block
    let dct_blocks: Vec<DCTBlock> = blocks.iter()
        .map(|block| discrete_cosine_transform(block))
        .collect();

    Ok(dct_blocks)
}

/// Splits the component video into 2x2 blocks.
///
/// # Arguments
///
/// * `component_video` - A slice of YPbPr values representing the component video.
/// * `width` - The width of the video in pixels, must be even.
///
/// # Returns
///
/// A `Result` containing a vector of 2x2 ComponentBlocks if successful, or an `Error` if the dimensions are incorrect.
fn split_into_blocks(component_video: &[(f64, f32, f32)], mut width: usize) -> Result<Vec<ComponentBlock>, Box<dyn Error>> {
    //Ensure the video can be evenly divided into 2x2 blocks
    if width % 2 != 0 {
        width -= 1;
    }

    let mut blocks = Vec::new();

    //Iterate over the pixels in 2x2 chunks
    for y in (0..component_video.len()).step_by(width * 2) {
        for x in (0..width).step_by(2) {
            let block = [
                component_video[y + x],
                component_video[y + x + 1],
                component_video[y + width + x],
                component_video[y + width + x + 1],
            ];
            blocks.push(block);
        }
    }
    

    Ok(blocks)
}

/// Applies the Discrete Cosine Transform (DCT) to a 2x2 block of Y values and calculates the average Pb and Pr.
///
/// # Arguments
///
/// * `block` - A reference to a 2x2 block of YPbPr values.
///
/// # Returns
///
/// A DCTBlock representing the transformed coefficients a, b, c, d and average Pb, Pr.
fn discrete_cosine_transform(block: &[(f64, f32, f32)]) -> DCTBlock {
    let (y1, pb1, pr1) = block[0];
    let (y2, pb2, pr2) = block[1];
    let (y3, pb3, pr3) = block[2];
    let (y4, pb4, pr4) = block[3];

    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;

    let avg_pb = (pb1 + pb2 + pb3 + pb4) / 4.0;
    let avg_pr = (pr1 + pr2 + pr3 + pr4) / 4.0;

    (a, b, c, d, avg_pb, avg_pr)
}

/// Applies the inverse Discrete Cosine Transform (iDCT) to a vector of DCTBlocks.
///
/// # Arguments
///
/// * `dct_blocks` - A slice of DCTBlocks to be transformed back to their original pixel values.
///
/// # Returns
///
/// A `Result` containing a vector of 2x2 ComponentBlocks if successful, or an `Error` if not.
pub fn inverse_dct(dct_blocks: &[DCTBlock]) -> Result<Vec<ComponentBlock>, Box<dyn Error>> {
    let mut component_blocks = Vec::with_capacity(dct_blocks.len());

    for &(a, b, c, d, avg_pb, avg_pr) in dct_blocks {
        //Apply inverse DCT formulas to recover the original Y values
        let y1 = a - b - c + d;
        let y2 = a - b + c - d;
        let y3 = a + b - c - d;
        let y4 = a + b + c + d;

        //Each DCTBlock is converted back into a ComponentBlock
        let block = [
            (y1, avg_pb, avg_pr),
            (y2, avg_pb, avg_pr),
            (y3, avg_pb, avg_pr),
            (y4, avg_pb, avg_pr),
        ];

        component_blocks.push(block);
    }

    Ok(component_blocks)
}