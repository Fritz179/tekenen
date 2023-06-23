use preloader::*;

fn main() {
    preload("src/preloaded.rs", preload_object(vec![
        ("img8", preload_image("src/img/8.png"))
    ]));
}