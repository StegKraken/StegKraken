use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
mod tui;

mod cracking;



#[derive(Debug, StructOpt)]
#[structopt(name = " StegKraken", about = "Fast Steg Bruteforcer")]
/// Bruteforce Steganography tool for the modern world
/// - Discord https://discord.gg/gfwzV9U
/// - GitHub https://github.com/StegKraken/StegKracken
struct Opt {
    /// The image you want to crack.
    #[structopt()]
    #[structopt(parse(from_os_str))]
    image: PathBuf,

    /// Wordlist you want to use.
    #[structopt(short, long, use_delimiter = true)]
    #[structopt(parse(from_os_str))]
    wordlist: PathBuf,

    //Accessible mode. Turns off features which negatively affect screen readers.
    #[structopt(short, long)]
    accessible: bool,

    /// How many threads do you want to run?
    #[structopt(short, long, default_value = "100")]
    threads: usize,
}

fn main() {
    let opt = Opt::from_args();

    rayon::ThreadPoolBuilder::new()
        .num_threads(opt.threads)
        .build_global()
        .unwrap();

    let image: &str = &opt.image.into_os_string().into_string().unwrap();

    print_opening();

    cracking::run_steghide(&String::from(""), image);

    cracking::read_and_split_file(opt.wordlist, image).unwrap();
}

fn print_opening() {
    println!("StegKraken. \n-Discord (support) https://discord.gg/gfwzV9U \n-GitHub https://github.com/StegKraken/StegKracken")
}
