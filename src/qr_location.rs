use std::vec;

use image::{self, GenericImageView, GrayImage, Luma, Pixel};
use oxidized_image_processing::{conv_2d, Kernel};

//Bias here will be promising points that are slightly better than just plain edges
pub fn finder_mark_location(bias: Vec<(usize, usize)>, image: GrayImage) -> (usize, usize) {
    return (0, 0);
}

pub fn gradient_image_content(image: &GrayImage) -> Vec<Vec<(f32, f32)>> {
    let second_der_x_dir = conv_2d(&Kernel::sobel_x_dir(), image);
    let second_der_y_dir = conv_2d(&Kernel::sobel_y_dir(), image);

    let (cols, rows) = second_der_x_dir.dimensions();

    let mut result = vec![vec![(0.0, 0.0); cols as usize]; rows as usize];

    for row in 0..rows {
        for col in 0..cols {
            result[col as usize][row as usize] = (0.0, 0.0);
        }
    }

    return result;
}

// pub fn canny_edge_detector(image: &GrayImage) -> GrayImage {
//     return *image;
// }

//This will take the result of the above. Maybe will return the vector of biased points that I can use to find the locator marks
pub fn box_detector(image: &GrayImage) -> Vec<(usize, usize)> {
    return vec![(0, 0)];
}
