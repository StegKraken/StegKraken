use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use rayon::prelude::*;

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
    rayon::ThreadPoolBuilder::new().num_threads(150).build_global().unwrap();
    println!("{:?}", opt);
    read_and_split_file(opt.wordlist, opt.image).unwrap();
}

fn read_and_split_file(wordlist: PathBuf, image_path: PathBuf) -> io::Result<()>{ 
    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);
    let image: &str = &image_path.into_os_string().into_string().unwrap();


    
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
    let status = Command::new("steghide")
        .args(&["extract", "-sf", &image_name, "-p", &password, "-f"])
        .status()
        .expect("failed to execute process");

    
    if status.success() {
        println!("Correct passphrase found: {}", &password);
        std::process::exit(0);
    } else {
        println!("");
    }
}