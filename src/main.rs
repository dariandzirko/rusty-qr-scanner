use std::env;

mod otsu;
mod qr_location;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let img = image::open("src/images/wikipedia_qr.png")
        .unwrap()
        .to_luma8();
    img.save("sanity_check.png");

    let otsu_img = otsu::otsu(&img);

    let canny_image = qr_location::canny_edge_detector(&otsu_img);

    // let test_vec = qr_location::box_detector(&img);
}
