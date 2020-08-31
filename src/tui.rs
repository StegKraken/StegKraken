/// Terminal User Interface Module for RustScan
/// Defines macros to use

#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        use ansi_term::Colour::Red;
        println!("{} {}", Red.bold().paint("[!]"), $name);
    };
    // quiet mode
    ($name:expr, $quiet:expr) => {
        use ansi_term::Colour::Red;
        if !$quiet {
            println!("{} {}", Red.bold().paint("[!]"), $name);
        }
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        use ansi_term::Colour::Blue;
        println!("{} {}", Blue.bold().paint("[~]"), $name);
    };
    ($name:expr, $quiet:expr) => {
        use ansi_term::Colour::Blue;
        if !$quiet {
            println!("{} {}", Blue.bold().paint("[~]"), $name);
        }
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr, $quiet:expr) => {
        use ansi_term::Colour::RGB;
        if !$quiet {
            println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
        }
    };
    ($name:expr) => {
        use ansi_term::Colour::RGB;
        println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
    };
}
