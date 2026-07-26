> C. Wyatt Polasek & Zach Breene <br>
> Assignment 4 - arith <br>
> README.md

> ### 1. Acknowledgments

- TA's
- Lots of notes from class and TA's
- Github Copilot
- Rust Documentation and User Forums
- Rust Programming Language Book
- Stack Overflow Community
- CSC411 Course Materials
- https://docs.rs/csc411_image/latest/src/csc411_image/image.rs.html
- https://docs.rs/csc411_rpegio/latest/src/csc411_rpegio/input.rs.html
- https://github.com/ndaniels/csc411_arith/blob/main/src/lib.rs
- Lots of help came from the instructions on the Arith PDF

> ### 2. Correctly Implemented Options

We have completed all parts of the assignment listed below:

- Image compression and decompression using the RPEG codec
- Quantization and dequantization of DCT coefficients
- Efficient bit-packing for image data representation
- Conversion between RGB and YPbPr color spaces
- Implementation of the Discrete Cosine Transform and its inverse

All planned features have been implemented correctly to the best of our knowledge.

> ### 3. Architecture of Solutions

Our RPEG codec is designed around modularity and separation of concerns. The `codec` module is the entry point for the main compression and decompression processes, orchestrating the workflow across various modules. The `rgb_module` handles color space conversions, while the `image_module` manages block processing and DCT calculations. The `codeword_module` is responsible for the quantization and packing of image data into codewords. Each module encapsulates its logic and exposes functions for the `codec` module to use, ensuring a clean and maintainable codebase.

> ### 4. Time Spent

We spent approximately 10 hours analyzing the problems posed in the assignment, discussing different approaches, and designing our solution architecture.

After the initial analysis, we dedicated around 25-30 hours to implementing, testing, and refining our solution.

Usage
------
To use the RPEG codec for compressing an image, run:
cargo run -- -c image.ppm > imageoutput.rpeg

To decompress an image, use:
cargo run -- -d image.rpeg > imageoutput.ppm