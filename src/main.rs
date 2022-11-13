use std::env;

mod otsu;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let img = image::open("src/images/Boy.tif").unwrap().to_luma8();
    let thresh = otsu::otsu_threshold(&img);
    println!("Thresh: {:?}", thresh);
}
