<h1 align=center> Arith: Lossy Image Compression </h1>
<h2 align=center> A CSC 411: Computer Organization Assignment by C. Wyatt Polasek & Zach Breene </h2>
<h4 align=center> Created at the University of Rhode Island, November 2023 </h4>

## Introduction
The primary objective of this assignment was to implement a lossy image compressor in Rust, focusing on integer and logical operations to manipulate image data. The project requires unpacking and repacking representations that put multiple signed and unsigned integers into a 32-bit word, alongside utilizing two's-complement arithmetic. 

---

## Implementation + Functions
### arith/bitpack/src/bitpack.rs

This directory houses the custom bitpack crate used to pack and unpack bits into words, which is fundamental for the compression and decompression sequences. <br>

&emsp; ***Bit Manipulation Method***

* The interface provides functions to extract fields (`gets`, `getu`), update fields (`news`, `newu`), and test if an integer can be represented in a specific number of bits (`fitss`, `fitsu`).
* The functions are designed to raise checked run-time errors if the requested field widths or least significant bit (LSB) placements exceed the 64-bit bounds of the word.
* If a value cannot fit within the designated signed or unsigned width, the field-update functions return `None` to handle overflow and underflow conditions.

### arith/rpeg/src/

This project directory contains the core implementation of the lossy image compressor, divided into various modules to handle different compression steps. <br>

&emsp; ***Compression Algorithm Method***

* The `codec` module handles the high-level `compress` and `decompress` functions.
* The compressor reads a PPM image using the `csc411_image` crate, trimming the last row or column if the dimensions are not divisible by two.
* The image pixels are converted from an RGB color space into a component video color space representing luminance (Y) and color-difference signals (Pb and Pr).
* The pixels are grouped into 2-by-2 blocks, where the chroma elements (Pb and Pr) are averaged and quantized into 4-bit representations using an index table.
* A Discrete Cosine Transform (DCT) converts the four luminance values into cosine coefficients (a, b, c, d) representing the average brightness and spatial gradient shifts within the block.
* The quantized chroma indices and DCT coefficients are packed into a single 32-bit code word using the custom bitpack crate and written to standard output in big-endian order.

---

## Information Loss Analysis
Because this is a lossful compression algorithm, image data is permanently lost at several specific stages.
* Trimming odd-numbered rows or columns directly removes the outer edge of the image.
* Averaging the chroma data across a 2-by-2 block results in a loss of specific color information for individual pixels.
* Approximating or disregarding high-frequency components during the Discrete Cosine Transform to fit the 5-bit coefficients discards spatial detail.
* Repeatedly compressing and decompressing the same image compounds these quantization and truncation errors, further degrading the visual quality.

---

## How To Run
**IMPORTANT: Ensure you have a working Rust environment.**

* Navigate to the `arith/rpeg` directory to execute the program.
* The program accepts command-line arguments to either compress or decompress a provided PPM file or standard input stream.
* To compress an image, run the binary using the compress flag: `rpeg -c [filename]`.
* To decompress an image, run the binary using the decompress flag: `rpeg -d [filename]`.

---

## Contribution
* **Partners:** C. Wyatt Polasek and Zach Breene.
* **Design Methodology:** This project utilized a stepwise refinement approach, designing, implementing, and testing each component individually before integrating them into the final executable. Testing emphasized universal laws, such as ensuring that unpacked values identically matched the original input when repacked.
