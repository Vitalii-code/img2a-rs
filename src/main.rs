use image::{DynamicImage, GenericImageView, Pixel};
use std::env;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // if there is no args print error
    if args.len() <= 1 {
        eprintln!("No arguments");
    } else {
        for arg in args.iter().skip(1) {
            convert_to_ascii(&arg);
        }
    }
}

fn convert_to_ascii(image_path: &str) {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return eprintln!("{}", e),
    };

    let terminal_size = get_terminal_size();
    println!("{:?}", terminal_size);

    let palette: String = " .:-=+*#%@".to_string();
    //let palette =  r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,^`'. ".chars().rev().collect::<String>();
    let image_size = image.dimensions();

    // calculating cluster size
    let cluster_width = image_size.0 / terminal_size.0 as u32 * 3;
    let cluster_height = image_size.1 / terminal_size.1 as u32 * 2;

    let mut y = 0;
    while image_size.1 >= y + cluster_height {
        let mut x = 0;
        while image_size.0 >= x + cluster_width {
            let brightness = get_brightness_of_cluster(&image, x, y, cluster_width, cluster_height);

            print!("{}", pick_char_from_palette(brightness, 255, &palette));
            x += cluster_width;
        }
        println!();
        y += cluster_height;
    }
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

fn get_brightness_of_cluster(
    image: &DynamicImage,
    x: u32,
    y: u32,
    cluser_width: u32,
    cluster_height: u32,
) -> usize {
    // this function return medium brightness of cluster of pixels
    let mut list_of_lums = Vec::new(); // list that store lums of pixels

    for y in y..y + cluster_height {
        for i in x..x + cluser_width {
            let pixel = image.get_pixel(i, y).to_luma();
            list_of_lums.push(pixel.0[0]);
        }
    }

    let mut sum: usize = 0;
    for i in list_of_lums.iter() {
        sum = sum + *i as usize;
    }

    return sum / list_of_lums.len();
}

fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        panic!("Can't get terminal size")
    }
}
