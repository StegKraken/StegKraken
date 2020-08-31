use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Command};
use structopt::StructOpt;
use rayon::prelude::*;

mod tui;

#[derive(Debug, StructOpt)]
#[structopt(name = " StegKraken", about = "Fast Steg Bruteforcer")]
/// Bruteforce Steganography tool for the modern world
/// - Discord https://discord.gg/gfwzV9U
/// - GitHub https://github.com/StegKraken/StegKracken
struct Opt {
    /// The image you want to crack.
    #[structopt(short, long, use_delimiter = true)]
    #[structopt(parse(from_os_str))]
    image: PathBuf,

    /// Wordlist you want to use.
    #[structopt(short, long, use_delimiter = true)]
    #[structopt(parse(from_os_str))]
    wordlist: PathBuf,

    ///Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    quiet: bool,

    //Accessible mode. Turns off features which negatively affect screen readers.
    #[structopt(short, long)]
    accessible: bool,
}

fn main() {
    let opt = Opt::from_args();
    let image: &str = &opt.image.into_os_string().into_string().unwrap();
    let mut blank = "";

    rayon::ThreadPoolBuilder::new().num_threads(350).build_global().unwrap();
    run_steghide(blank, image);
    println!("{:?}", opt);
    read_and_split_file(opt.wordlist,image).unwrap();
}

fn read_and_split_file(wordlist: PathBuf, image_path: &str) -> io::Result<()>{ 
    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);
    let image = image_path;
    
    let mut buffer: std::vec::Vec<String> = Vec::new(); 
    for line in reader.lines() {
        buffer.push(line?);
        if buffer.len() == 100{
            crack_batch(&buffer, image);
        }
    }

    if buffer.len() > 0 || buffer.len() < 100 {
        crack_batch(&buffer, image);
    }
    Ok(())
}

fn crack_batch(batch: &Vec<String>, image_path: &str) {
    batch.into_par_iter().for_each(|x| run_steghide(x, image_path));
}

fn run_steghide(password: &String, image_name: &str,) {
    // TODO check if command returns Ok
    let output= Command::new("steghide")
        .args(&["extract", "-sf", &image_name, "-p", &password, "-f"])
        .output()
        .expect("failed to execute process");

    // println!("process exited with: {}", output.status);

    if output.status.success() {
        detail!(format!("Correct passphrase found: {}", &password));
        println!("Data extracted to current directory.");
        std::process::exit(0);
    } else {
    }
}