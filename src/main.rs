mod convert;
use crate::convert::to_ascii;
use arboard::Clipboard;
use clap::Parser;
#[cfg(test)]
mod test;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specifies the custom palette for visual representation.
    #[arg(short, long, default_value_t=String::from(" .:-=+*#%@"))]
    palette: String,

    /// Makes your image colored
    #[arg(long, visible_alias = "color", default_value_t = false)]
    colour: bool,

    /// Copies your converted image to your clipboard
    #[arg(short, long, short = 'c', default_value_t = false)]
    copy: bool,

    /// Path to your image or images
    file: Vec<String>,
}

fn main() {
    // collect args
    let cli = Cli::parse();
    if cli.file.is_empty() {
        eprintln!(
            "You need to input your image(s). Please run 'img2a --help' for more information."
        );
    };

    // iterate through images
    for image in cli.file.iter() {
        match to_ascii(&image, &cli.palette, cli.colour) {
            Ok(ascii) => {
                if cli.copy {
                    match Clipboard::new() {
                        Ok(mut clipboard) => clipboard.set_text(&ascii).unwrap(),
                        Err(e) => eprintln!("{}", e),
                    };
                } else {
                    println!("{}", ascii)
                }
            }
            Err(e) => eprintln!("{} occurred during convertation of '{}'", e, image),
        };
    }
}
