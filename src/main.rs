use image::{DynamicImage, GenericImageView, Pixel};
use std::env;
use terminal_size::{terminal_size, Height, Width};

#[cfg(test)]
mod test;

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // if there is no args print error
    if args.len() <= 1 {
        eprintln!("No arguments");
    } else {
        for arg in args.iter().skip(1) {
            to_ascii(&arg);
        }
    }
}

struct Cluster {
    width: u32,
    height: u32,
}

fn calculate_clustersize(image_size: (u32, u32)) -> Cluster {
    // calculating cluster size
    let terminal_size = get_terminal_size();

    // we need to get image resolution ratio
    let ratio = image_size.0 as f64 / image_size.1 as f64;

    let cluster = Cluster {
        width: (image_size.0 as f64 / terminal_size.0 as f64 * ratio) as u32,
        height: image_size.1 / terminal_size.1 as u32,
    };

    return cluster;
}

fn to_ascii(image_path: &str) {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return eprintln!("{}", e),
    };

    let palette: String = " .:-=+*#%@".to_string();
    let image_size = image.dimensions();
    let cluster: Cluster = calculate_clustersize(image_size);

    let mut y = 0;
    while image_size.1 >= y + cluster.height {
        let mut x = 0;
        while image_size.0 >= x + cluster.width {
            let brightness = get_brightness_of_cluster(&image, x, y, cluster.width, cluster.height);

            print!("{}", pick_char_from_palette(brightness, 255, &palette));
            x += cluster.width;
        }
        println!();
        y += cluster.height;
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
    cluster_width: u32,
    cluster_height: u32,
) -> usize {
    // this function return medium brightness of cluster of pixels
    let list_size: usize = (cluster_width * cluster_height) as usize;
    let mut list_of_lums = Vec::with_capacity(list_size);

    for y in y..y + cluster_height {
        for x in x..x + cluster_width {
            let pixel = image.get_pixel(x, y).to_luma();
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

