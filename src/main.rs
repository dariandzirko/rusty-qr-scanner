use std::env;

mod otsu;
mod qr_location;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let img = image::open("src/images/Boy.tif").unwrap().to_luma8();

    let otsu_img = otsu::otsu(&img);
    otsu_img.save("Otsu_Boy.tif");

    let test_vec = qr_location::box_detector(&img);
}
