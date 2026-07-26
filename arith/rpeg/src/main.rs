//C. Wyatt Polasek + Zach Breene
//Rpeg Main
//Taken from PDF

use std::env;
use rpeg::codec::{compress, decompress};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    match args[1].as_str() {
        "-c" => {let _ = compress(Some(filename));}
        "-d" => {let _ = decompress(Some(filename));}
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}
