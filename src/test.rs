use image::flat::Error;
use super::*;

#[test]
fn my_test() {
    for i in 1..2000 {
        generate("imgs/image.png", (i, 500)).unwrap();
        to_ascii("imgs/image.png");
    }
}


fn generate(name: &str, image_size: (u32, u32)) -> Result<(),Error>{
    // This function generates an image for testing

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_size.0, image_size.1);

    // A redundant loop to demonstrate reading image data
    for x in 0..image_size.0 {
        for y in 0..image_size.1 {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], 0 as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(name).unwrap();
    
    Ok(())
}
