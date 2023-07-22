use std::fs;

use image::{io::Reader as ImageReader, GenericImageView};

fn parse_image(path: &str) -> Vec<u8> {
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    let mut vec = vec![];

    let width = img.width().to_be_bytes();
    let height = img.height().to_be_bytes();

    vec.extend_from_slice(&width);
    vec.extend_from_slice(&height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let color = img.get_pixel(x, y);
            vec.push(color[0]);
            vec.push(color[1]);
            vec.push(color[2]);
            vec.push(color[3]);
        }
    };

    vec
}

fn main() {
    let data = parse_image("./src/img/8.png");

    fs::write("./src/img/8.bin", &data).unwrap();
}