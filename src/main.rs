use colored::*;
use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use std::env;
use terminal_size::{terminal_size, Height, Width};

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
                let ascii = to_ascii(&arg, colour);
                println!("{}", ascii)
            }
        }
    }
}
#[derive(Debug)]
struct Cluster {
    width: u32,
    height: u32,
}

fn calculate_clustersize(image_size: (u32, u32)) -> Cluster {
    // someone please fix this function!!!

    // calculating cluster size
    let terminal_size = get_terminal_size();

    if terminal_size.0 as u32 >= image_size.0 || terminal_size.1 as u32 >= image_size.1 {
        return Cluster {
            width: 1,
            height: 2,
        };
    }

    let cluster = Cluster {
        width: image_size.0 / terminal_size.0 as u32 * 2,
        height: image_size.1 / terminal_size.1 as u32 * 2,
    };

    return cluster;
}

fn to_ascii(image_path: &str, colour: bool) -> String {
    let mut ascii = String::new();

    let image = image::open(image_path).unwrap();

    let palette = String::from(" .:-=+*#%@");
    let image_size = image.dimensions();
    let cluster: Cluster = calculate_clustersize(image_size);

    let mut y = 0;
    while image_size.1 >= y + cluster.height {
        let mut x = 0;
        while image_size.0 >= x + cluster.width {
            let brightness = get_brightness_of_cluster(&image, x, y, cluster.width, cluster.height);

            let letter = pick_char_from_palette(brightness, 255, &palette);
            if colour == true {
                let rgb = get_colour_of_cluster(&image, x, y, cluster.width, cluster.height);
                let colored_letter = letter.to_string().truecolor(
                    rgb.0[0].try_into().unwrap(),
                    rgb.0[1].try_into().unwrap(),
                    rgb.0[2].try_into().unwrap(),
                );
                ascii = format!("{}{}", ascii, colored_letter);
            } else if colour == false {
                ascii.push(letter);
            };
            x += cluster.width;
        }
        ascii.push('\n');
        y += cluster.height;
    }

    return ascii;
}

fn pick_char_from_palette(value: usize, max_value: usize, palette: &String) -> char {
    if value > max_value {
        panic!("The value is higher than the max value")
    } else {
        let mut char = max_value as f64 / palette.len() as f64;
        char = char.ceil();
        let char = value as usize / char as usize;

        return palette.chars().nth(char).unwrap();
    }
}

fn get_colour_of_cluster(
    image: &DynamicImage,
    x: u32,
    y: u32,
    cluster_width: u32,
    cluster_height: u32,
) -> Rgb<u32> {
    // this function return medium colour of cluster of pixels

    let list_size: usize = (cluster_width * cluster_height) as usize;
    let mut list_of_colours: Vec<Rgb<u8>> = Vec::with_capacity(list_size);

    for y in y..y + cluster_height {
        for x in x..x + cluster_width {
            let pixel_colour = image.get_pixel(x, y).to_rgb();
            list_of_colours.push(pixel_colour);
        }
    }

    let mut sums: (u32, u32, u32) = (0, 0, 0);
    for pixel in list_of_colours.iter() {
        sums.0 = sums.0 + pixel.0[0] as u32;
        sums.1 = sums.1 + pixel.0[1] as u32;
        sums.2 = sums.2 + pixel.0[2] as u32;
    }
    let avg_r: u32 = (sums.0 as usize / list_size).try_into().unwrap();
    let avg_g: u32 = (sums.1 as usize / list_size).try_into().unwrap();
    let avg_b: u32 = (sums.2 as usize / list_size).try_into().unwrap();

    return Rgb::from([avg_r, avg_g, avg_b]);
}

fn get_brightness_of_cluster(
    image: &DynamicImage,
    x: u32,
    y: u32,
    cluster_width: u32,
    cluster_height: u32,
) -> usize {
    // this function return medium brightness of cluster of pixels
    let list_size: usize = (cluster_width * cluster_height) as usize;
    let mut list_of_lums: Vec<u32> = Vec::with_capacity(list_size);

    for y in y..y + cluster_height {
        for x in x..x + cluster_width {
            let pixel = image.get_pixel(x, y).to_luma();
            list_of_lums.push(pixel.0[0] as u32);
        }
    }

    let sum: u32 = list_of_lums.iter().sum();
    return sum as usize / list_size;
}

fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        panic!("Can't get terminal size")
    }
}
