//C. Wyatt Polasek + Zach Breene
//Rpeg Codec Codeword Module

use bitpack::bitpack::{fitsu, newu};
use csc411_arith::{index_of_chroma, chroma_of_index};
use std::error::Error;

type DctBlock = (f64, f64, f64, f64, f32, f32); //a, b, c, d, avg_pb, avg_pr

/// Quantizes and packs data from DCT blocks into 32-bit codewords.
///
/// # Arguments
///
/// * `dct_blocks` - A vector of `DctBlock` containing the DCT coefficients and chroma values.
///
/// # Returns
///
/// A `Result` containing a vector of `u32` codewords if successful, or an `Error` if not.
pub fn quantize_and_pack(dct_blocks: &Vec<DctBlock>) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut codewords = Vec::new();

        for block in dct_blocks.iter() {
            let (a, b, c, d, pb, pr) = *block;

            //Quantize the coefficients using provided methods
            let quantized_a = quantize_coefficient_a(a); //Function to quantize 'a' 
            let quantized_b = quantize_coefficients_bcd(b); //Function to quantize 'b'
            let quantized_c = quantize_coefficients_bcd(c); //Function to quantize 'c'
            let quantized_d = quantize_coefficients_bcd(d); //Function to quantize 'd'

            //Quantize chroma using csc411_arith
            let pb_index = index_of_chroma(pb);
            let pr_index = index_of_chroma(pr);

            //Ensure the values fit into their respective fields
            if !fitsu(pb_index as u64, 4) || !fitsu(pr_index as u64, 4) {
                return Err("Chroma index out of bounds".into());
            }

            //Pack the values into a 32-bit word
            let mut codeword = 0u32;
            codeword = newu(codeword as u64, 9, 23, quantized_a as u64).ok_or("Value does not fit in 9 bits")? as u32;
            codeword = newu(codeword as u64, 5, 18, quantized_b as u64).ok_or("Value does not fit in 5 bits")? as u32;
            codeword = newu(codeword as u64, 5, 13, quantized_c as u64).ok_or("Value does not fit in 5 bits")? as u32;
            codeword = newu(codeword as u64, 5, 8, quantized_d as u64).ok_or("Value does not fit in 5 bits")? as u32;
            codeword = newu(codeword as u64, 4, 4, pb_index as u64).ok_or("Value does not fit in 4 bits")? as u32;
            codeword = newu(codeword as u64, 4, 0, pr_index as u64).ok_or("Value does not fit in 4 bits")? as u32;

            codewords.push(codeword);        
        }
    Ok(codewords)
}

/// Quantizes a single DCT coefficient.
///
/// # Arguments
///
/// * `coef` - A `f64` value representing the DCT coefficient to be quantized.
///
/// # Returns
///
/// A `i32` value representing the quantized coefficient.
fn quantize_coefficient_a(coef: f64) -> u64 {
    //Assuming the coefficient 'a' is between -1 and 1, we scale it to fit in 9 bits
    ((coef * 511.0).round() as i64).rem_euclid(512) as u64 // rem_euclid ensures we get a positive value in the range [0, 511]
}
fn quantize_coefficients_bcd(coef: f64) -> u64 {
    //Clamping the coefficients 'b', 'c', 'd' between -0.3 and 0.3 and scaling them to fit in 5 bits
    let clamped = coef.clamp(-0.3, 0.3);
    let scaled = ((clamped * 50.0).round() as i64).rem_euclid(32) as u64; //rem_euclid ensures we get a positive value in the range [0, 31]
    scaled
}

/// Unpacks and dequantizes codewords into DCT blocks.
///
/// # Arguments
///
/// * `codewords` - A vector of `[u8; 4]` each representing a packed codeword.
///
/// # Returns
///
/// A `Result` containing a vector of `DctBlock` if successful, or an `Error` if not.
pub fn unpack_and_dequantize(codewords: &Vec<[u8; 4]>) -> Result<Vec<DctBlock>, Box<dyn Error>> {
    let mut dct_blocks = Vec::with_capacity(codewords.len());

    for &codeword_bytes in codewords {
        //Convert the bytes to a 32-bit integer
        let codeword = u32::from_be_bytes(codeword_bytes);

        //Extract the quantized coefficients and indices
        let quantized_a = ((codeword >> 23) & 0x01FF) as i32;
        let quantized_b = ((codeword >> 18) & 0x001F) as i32;
        let quantized_c = ((codeword >> 13) & 0x001F) as i32;
        let quantized_d = ((codeword >> 8) & 0x001F) as i32;
        let pb_index = ((codeword >> 4) & 0x000F) as usize;
        let pr_index = (codeword & 0x000F) as usize;

        //Dequantize coefficients
        let a = dequantize_coefficient_a(quantized_a);
        let b = dequantize_coefficients_bcd(quantized_b);
        let c = dequantize_coefficients_bcd(quantized_c);
        let d = dequantize_coefficients_bcd(quantized_d);

        //Convert indices back to Pb and Pr values
        let pb = chroma_of_index(pb_index);
        let pr = chroma_of_index(pr_index);

        //Form the DCT block from the dequantized values
        let dct_block = (a, b, c, d, pb, pr);

        dct_blocks.push(dct_block);
    }

    Ok(dct_blocks)
}

/// Dequantizes a single coefficient.
///
/// # Arguments
///
/// * `quantized_val` - An `i32` value representing the quantized coefficient.
///
/// # Returns
///
/// A `f64` value representing the dequantized coefficient.
fn dequantize_coefficient_a(quantized_val: i32) -> f64 {
    //Convert the quantized integer back to the floating-point value and reverse the scaling
    quantized_val as f64 / 511.0
}

fn dequantize_coefficients_bcd(quantized_val: i32) -> f64 {
    //Convert the quantized integer back to the floating-point value, reverse the scaling and clamping
    quantized_val as f64 / 50.0
}
