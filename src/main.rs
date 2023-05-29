use std::env;

use otsu::otsu;

mod otsu;
mod qr_location;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");

    let img = image::open("src/images/Plane.jpg").unwrap().to_luma8();
    img.save("sanity_check.png");

    let otsu_img = otsu::otsu(&img);
    otsu_img.save("otsu_img.png");

    let canny_image = qr_location::canny_edge_detector(&img);
    canny_image.save("double_thresh_image.png");

    // let test_vec = qr_location::box_detector(&img);
}
