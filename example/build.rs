use std::fs;
use tekenen::preloader::parse_image;

fn main() {
    let data = parse_image("./src/img/8.png");

    fs::write("./src/img/8.fpia", &data).unwrap();
}