use colored::*;
use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use terminal_size::{terminal_size, Height, Width};

#[derive(Debug)]
struct Cluster {
    width: u32,
    height: u32,
}

pub fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        panic!("Can't get terminal size")
    }
}

pub fn to_ascii(
    image_path: &str,
    palette: &String,
    colour: bool,
) -> Result<String, image::ImageError> {
    let mut ascii = String::new();

    let image = image::open(image_path)?;

    let image_size = image.dimensions();
    let cluster: Cluster = calculate_clustersize(image_size);

    let mut y = 0;
    while image_size.1 >= y + cluster.height {
        let mut x = 0;
        while image_size.0 >= x + cluster.width {
            let mut brightness =
                get_brightness_of_cluster(&image, x, y, cluster.width, cluster.height);
            if colour {
                brightness = 0
            }

            let letter = pick_char_from_palette(brightness, 255, &palette);
            if colour {
                let rgb = get_colour_of_cluster(&image, x, y, cluster.width, cluster.height);
                let colored_letter = letter.to_string().on_truecolor(
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

    return Ok(ascii);
}

fn get_colour_of_cluster(
    image: &DynamicImage,
    x: u32,
    y: u32,
    cluster_width: u32,
    cluster_height: u32,
) -> Rgb<u32> {
    // this function return an average colour of cluster of pixels

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
        height: image_size.1 / terminal_size.1 as u32 * 1,
    };

    return cluster;
}

fn pick_char_from_palette(index: usize, max_index: usize, palette: &String) -> char {
    if index > max_index {
        panic!("The index is higher than the max index");
    } else {
        let palette_length = palette.len();
        let index_in_palette = max_index / palette_length;
        return palette.chars().nth(index / index_in_palette).unwrap_or(' ');
    }
}
