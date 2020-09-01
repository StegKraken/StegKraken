use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

pub fn read_and_split_file(wordlist: PathBuf, image_path: &str) -> io::Result<()> {
    let file = File::open(wordlist)?;
    let file = BufReader::new(file);

    let mut strings_vector: std::vec::Vec<String> = Vec::new();
    //let mut stringsVector = Vec::new();


    let mut reader = BufReader::new(file);
    let mut buf = vec![];

    while let Ok(_) = reader.read_until(b'\n', &mut buf) {
        if buf.is_empty() {
            break;
        }
        let line = String::from_utf8_lossy(&buf);
        strings_vector.push(line.to_string());
        if strings_vector.len() > 99 {
            crack_batch(&strings_vector, image_path);
            buf.clear();
            strings_vector.clear();
        }
        buf.clear();
    }
    // if wordlist is exhausted but we still have passwords
    // try them
    // this helps us if the wordlist is uneven
    if strings_vector.len() > 0{
        crack_batch(&strings_vector, image_path);
    }

    // wordlist is exhausted!
    println!("Wordlist exhausted. The password wasn't found in this wordlist.");

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
