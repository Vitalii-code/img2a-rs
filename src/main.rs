use std::env;
mod convert;
use crate::convert::to_ascii;
#[cfg(test)]
mod test;

const HELP: &str = "
Usage: img2a [option] [images...]

Options:
  -h, --help
          Print help information

  --colour, --color
          By default the output will be colourless, use this flag to add colour
";

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // if there is no args print error
    if args.len() <= 1 {
        eprintln!("No arguments");
        println!("{}", HELP);
    } else {
        let mut colour = false;
        for arg in args.iter().skip(1) {
            if arg == "--colour" || arg == "--color" {
                colour = true
            } else if arg == "-h" || arg == "--help" {
                println!("{}", HELP);
            } else {
                match to_ascii(&arg, colour) {
                    Ok(ascii) => println!("{}", ascii),
                    Err(e) => eprintln!("{} occurred during convertation of '{}'", e, arg),
                };
            }
        }
    }
}
