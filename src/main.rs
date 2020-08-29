use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

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
    println!("{:?}", opt);
    read_and_split_file(opt.wordlist, opt.image);
    println!("Just read file");
    /*let _stdin = std::io::stdin();
    let _n = 20;
    let mut vec = vec![];

    let wordlist = "rockyou.txt";
    let mut _file = File::open(wordlist).expect("Error, can't open file");
    let _reader = BufReader::new(_file);

    for line in &_reader.lines().into_iter().chunks(1) {
        vec.insert(0, line);
    }*/
}

fn read_and_split_file(wordlist: PathBuf, image_path: PathBuf) -> io::Result<()>{ 
    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);
    let image: &str = &image_path.into_os_string().into_string().unwrap();


    
    let mut buffer: std::vec::Vec<String> = Vec::new(); 
    for line in reader.lines() {
        buffer.push(line?);
        if buffer.len() == 100{
            crack_batch(buffer, image);
        }
    }
    Ok(())
}

fn crack_batch(batch: Vec<String>, image_path: &str) {
    batch.iter().map(|x| run_steghide(x, image_path));
}

fn run_steghide(password: &String, image_name: &str,) {
    Command::new("steghide")
        .args(&["extract", "-sf", &image_name, "-p", &password, "-f"])
        .output()
        .expect("failed to execute process");
}
