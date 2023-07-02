use image::{DynamicImage, GenericImageView, Pixel};
use std::env;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        eprintln!("No arguments");
    } else {
        let image = image::open(&args[1]).unwrap();
        let terminal_size = get_terminal_size();
        let palette: String = " .:-=+*#%@".to_string();
        //let palette =  r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,^`'. ".chars().rev().collect::<String>();
        let image_size = image.dimensions();

        // calculating cluster size
        let cluster_width = image_size.0 / terminal_size.0 as u32 * 6;
        let cluster_height = image_size.0 / terminal_size.0 as u32 * 8;
        println!("{:?}", terminal_size);

        let mut y = 0;
        while image_size.1 >= y + cluster_height {
            let mut x = 0;
            while image_size.0 >= x + cluster_width {
                let brightness =
                    get_brightness_of_cluster(&image, x, y, cluster_width, cluster_height);

                print!("{}", pick_char_from_palette(brightness, 255, &palette));
                x += cluster_width;
            }
            println!();
            y += cluster_height;
        }
    }
}

fn pick_char_from_palette(value: usize, max_value: usize, palette: &String) -> char {
    if value > max_value {
        panic!("The value is higher the max value")
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

// use clearscreen;
// use std::{thread, time};

// fn spawn_pyramid(angle:usize, symbol:&str, blank:&str) {
//     for i in 0..10 {
//         println!("{}", blank.repeat(i*angle)+&symbol.repeat(i))
//     }
// }

// fn main() {

//     const DELAY_IN_MILLS:u64 = 500;
//     const MAX_ANGLE:usize = 1;
//     const MIN_ANGLE:usize = 0;
//     const STEP:usize = 1;
//     let mut angle:usize = 0;
//     let mut side:bool = true;

//     loop {

//         spawn_pyramid(angle, "_", "!");

//         if side {
//             angle = angle + STEP;
//             if angle > MAX_ANGLE {
//                 side = false;
//             }
//         } else {
//             angle = angle - STEP;
//             if angle <= MIN_ANGLE {
//                 side = true;
//             }
//         }

//         thread::sleep(time::Duration::from_millis(DELAY_IN_MILLS));
//         clearscreen::clear().expect("failed to clear screen");

//     }
// }
