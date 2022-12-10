use std::vec;

use image::{self, GenericImageView, GrayImage, Luma, Pixel};
use oxidized_image_processing::{conv_2d, Kernel};

//Bias here will be promising points that are slightly better than just plain edges
pub fn finder_mark_location(bias: Vec<(usize, usize)>, image: GrayImage) -> (usize, usize) {
    return (0, 0);
}

pub fn gradient_image_content(image: &GrayImage) -> Vec<(f32, f32)> {
    //This is incorrect atm because it does not return the same size image
    let second_der_x_dir = conv_2d(&Kernel::sobel_x_dir(), image, true);
    let second_der_y_dir = conv_2d(&Kernel::sobel_y_dir(), image, true);

    second_der_x_dir.save("second_der_x_dir.png");
    second_der_y_dir.save("second_der_y_dir.png");

    let (cols, rows) = second_der_x_dir.dimensions();

    let mut result = vec![(0.0, 0.0); (rows * cols) as usize];

    for row in 0..rows {
        for col in 0..cols {
            let gx = *second_der_x_dir
                .get_pixel(col, row)
                .channels()
                .get(0)
                .unwrap();
            let gy = *second_der_y_dir
                .get_pixel(col, row)
                .channels()
                .get(0)
                .unwrap();
            let magnitude_gradient =
                f32::sqrt(f32::powf(gx as f32, 2.0) + (f32::powf(gy as f32, 2.0)));
            let angle_gradient = (gy as f32).atan2(gx as f32);
            result[(col+(row*rows)) as usize] = (magnitude_gradient, angle_gradient);
        }
    }

    return result;
}

pub struct EdgeLine {
    name: String, //This will be interpretation of the name of each direction of edges not the name of the normal of the direction
    adjacent1: (i32, i32), //first adjacent pixel in the direction of the edge normal
    adjacent2: (i32, i32) //second adjancent pixel in the direction of the edge normal
}

pub fn normal_to_direction(angle: f32) -> EdgeLine {
    match angle {
        direction
            //-pi/8 .. pi/8
            if (-std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8)
                .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
        direction
            //pi/8 .. 3pi/8
            if (std::f32::consts::FRAC_PI_8 ..= std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, 1)},
        direction 
            //3pi/8 .. 5pi/8
            if (std::f32::consts::FRAC_PI_8 * 3.0 ..= std::f32::consts::FRAC_PI_8 * 5.0)
                .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
        direction 
            //5pi/8 .. 7pi/8
            if (std::f32::consts::FRAC_PI_8 * 5.0 ..= std::f32::consts::FRAC_PI_8 * 7.0)
                .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},

        direction
            //7pi/8 .. -7pi/8
            if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 7.0)
                .contains(&angle) => EdgeLine{name: "vertical_edge".to_owned(), adjacent1: (1, 0), adjacent2: (-1, 0)},
        direction
            //-7pi/8 .. -5pi/8
            if (-std::f32::consts::FRAC_PI_8 * 7.0 ..= -std::f32::consts::FRAC_PI_8 * 5.0)
                .contains(&angle) => EdgeLine{name: "neg_45_edge".to_owned(), adjacent1: (1, 1), adjacent2: (-1, 1)},
        direction 
            //-5pi/8 .. -3pi/8
            if (-std::f32::consts::FRAC_PI_8 * 5.0 ..= -std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle) => EdgeLine{name: "horizontal_edge".to_owned(), adjacent1: (0, 1), adjacent2: (0, -1)},
        direction 
            //-3pi/8 .. -pi/8
            if (-std::f32::consts::FRAC_PI_8 * 3.0 ..= -std::f32::consts::FRAC_PI_8)
                .contains(&angle) => EdgeLine{name: "pos_45_edge".to_owned(), adjacent1: (-1, 1), adjacent2: (1, -1)},
                
        _ => EdgeLine{name: "broken".to_owned(), adjacent1: (100, 100), adjacent2: (-100, -100)},
    }
}

//This is certainly wrong 
pub fn non_maxima_suppression(gradient_info: Vec<(f32, f32)>, cols: u32, rows: u32) -> GrayImage{
    let mut result = GrayImage::new(cols, rows);

    //I think this is correct, where I ignore the outermost border of pixels
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let (mag, angle) = gradient_info[(col+(row*rows)) as usize];
            let direction = normal_to_direction(angle);

            if mag > gradient_info[ ((col as i32 + direction.adjacent1.0) + (row as i32 + direction.adjacent1.1)*rows as i32) as usize].0 
            && mag > gradient_info[ ((col as i32 + direction.adjacent2.0) + (row as i32 + direction.adjacent1.1)*rows as i32) as usize].0 {
                let pixel = Luma([mag as u8]);
                result.put_pixel(col as u32, row as u32, pixel);
            }
        }
    }

    return result;
}

pub fn canny_edge_detector(image: &GrayImage) -> GrayImage {
    //This is incorrect atm because it does not return the same size image
    let smoothed_image = conv_2d(&Kernel::gaussian_2d(3.0), image, true);
    smoothed_image.save("smoothed_image.png");

    let smoothed_gradient = gradient_image_content(&smoothed_image);
    let (cols, rows) = smoothed_image.dimensions();

    let nom_maxima_suppressed_image = non_maxima_suppression(smoothed_gradient, cols, rows);

    return nom_maxima_suppressed_image;
}

//This will take the result of the above. Maybe will return the vector of biased points that I can use to find the locator marks
pub fn box_detector(image: &GrayImage) -> Vec<(usize, usize)> {
    return vec![(0, 0)];
}
