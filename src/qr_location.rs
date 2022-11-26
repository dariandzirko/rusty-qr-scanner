use std::vec;

use image::{self, GenericImageView, GrayImage, Luma, Pixel, Rgba};
use oxidized_image_processing::{conv_2d, Kernel};

//Bias here will be promising points that are slightly better than just plain edges
pub fn finder_mark_location(bias: Vec<(usize, usize)>, image: GrayImage) -> (usize, usize) {
    return (0, 0);
}

//This will give me an edge detected copy of the original image, making the initial search for boxes hopefully trivial
pub fn edge_detected_image(image: &GrayImage) -> GrayImage {
    conv_2d(&Kernel::sobel(), image)
}

//This will take the result of the above. Maybe will return the vector of biased points that I can use to find the locator marks
pub fn box_detector(image: &GrayImage) -> Vec<(usize, usize)> {
    let edge_detected_image = edge_detected_image(image);

    return vec![(0, 0)];
}
