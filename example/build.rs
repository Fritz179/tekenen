use std::fs;
use tekenen::preloader::parse_image;

fn main() {
    let data = parse_image("./src/img/8.png");

    fs::write("./src/img/8.fpia", data).unwrap();

    use std::env;
    
    for (key, value) in env::vars() {
        if key.starts_with("CARGO_") {
            println!("{}: {:?}", key, value);
        }
    }

    println!("{}: {:?}", "MYYYYYYYYYY", env::var("CARGO_CFG_TARGET_FAMILY").unwrap());


    #[cfg(all(target_arch = "wasm32"))]
    panic!("stop and dump stdout WASMM");

    #[cfg(target_arch = "wasm32")]
    panic!("stop and dump stdout WASM");

    #[cfg(target_family = "wasm")]
    panic!("stop and dump stdout WASM2");

    // #[cfg(target_family = "unix")]
    // panic!("stop and dump stdout unix");

    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "wasm" {
        println!("HELLOOOOOOOOOOO");

        #[cfg(target_family = "wasm")]
        println!("target_family = wasm");

        #[cfg(target_family = "unix")]
        println!("target_family = unix");


        // panic!("PANICCCCCCCC");
    }

    // panic!("stop and dump stdout DEFAULTT");
}