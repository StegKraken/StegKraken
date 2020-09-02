use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

pub fn read_and_split_file(wordlist: PathBuf, image_path: &str) -> io::Result<()> {
    let file = match File::open(wordlist){
        Ok(x) => {x}
        Err(y) => {panic!("This errors")}
    };
    let reader = BufReader::new(file);
    let image = image_path;
    let mut counter: u64 = 0;

    let mut buffer: std::vec::Vec<String> = Vec::new();
    for line in reader.lines() {
        match line{
            Ok(x) => {
                //println!("{}", x);
                buffer.push(x);
            }
            Err(_) => {}
        }
        if buffer.len() > 100_000 {
            counter = counter + 100_000;
            println!("{}", counter);
            crack_batch(&buffer, image);
            buffer.clear();
        }
    }
    println!("Yup sir, I getz it.");
    println!("Counter is {}", counter);


    if buffer.len() > 0 || buffer.len() < 1000 {
        crack_batch(&buffer, image);
    }
    
    Ok(())
}

fn crack_batch(batch: &Vec<String>, image_path: &str) {
    batch
        .into_par_iter()
        .for_each(|x| run_steghide(x, image_path));
}

pub fn run_steghide(password: &String, image_name: &str) {
    let output = Command::new("steghide")
        .args(&["extract", "-sf", &image_name, "-p", &password, "-f"])
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        detail!(format!("Correct passphrase found: {}", &password));
        println!("Data extracted to current directory.");
        std::process::exit(0);
    } else {
    }
}
