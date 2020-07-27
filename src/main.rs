use std::fs::File;
use std::io;
use itertools::Itertools;
use std::io::BufRead;
use std::io::prelude::*;
use std::io::BufReader;



fn main() {
    let _stdin = std::io::stdin();
    let _n = 20;
    let mut vec = vec![];

	let wordlist = "rockyou.txt";
	let mut _file = File::open(wordlist).expect("Error, can't open file");
    let _reader = BufReader::new(_file);

    for line in &_reader.lines().into_iter().chunks(1) {
    	vec.insert(0, line);
    }
}
