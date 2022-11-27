use std::env;

mod otsu;
mod qr_location;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let img = image::open("src/images/wikipedia_qr.png")
        .unwrap()
        .to_luma8();
    img.save("sanity_check.png");

    let edge_detected_img = qr_location::edge_detected_image(&img);
    edge_detected_img.save("edge_detected.png");

    //let otsu_img = otsu::otsu(&img);

    // let test_vec = qr_location::box_detector(&img);
}
