use image::{self, GenericImageView, GrayImage, Luma, Pixel, Rgba};

struct GrayHistogram {
    histogram: [u32; 256],
    probabilities: [f32; 256],
}

fn make_grayhistrogram(image: &GrayImage) -> GrayHistogram {
    //ni
    let mut histogram = [0; 256];
    //pi = ni/MN where MN = width*height = total number of pixels in an images
    let mut probabilities = [0.0; 256];

    let (width, height) = image.dimensions();
    let sum = width * height;

    for pixel in image.pixels() {
        histogram[*pixel.channels().get(0).unwrap() as usize] += 1;
    }

    probabilities
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x = *x + (histogram[i] as f32 / sum as f32));

    GrayHistogram {
        histogram,
        probabilities,
    }
}

pub fn otsu_threshold(image: &GrayImage) -> usize {
    let easy_histogram = make_grayhistrogram(image);

    //q1(k)
    let mut probabilities_class1 = [0.0; 256];

    //m1(k)
    let mut mean_intensities_class1 = [0.0; 256];

    for i in 0..256 {
        let mut sum = 0.0;
        let mut sum_probabilities = 0.0;

        for j in 0..i + 1 {
            sum += easy_histogram.probabilities[j];
            sum_probabilities += easy_histogram.probabilities[j] * j as f32;
        }

        probabilities_class1[i] = sum;
        mean_intensities_class1[i] = sum_probabilities;
    }

    //mg
    let global_mean_intensity = *mean_intensities_class1.last().unwrap();

    //sigmab^2
    let mut between_class_var = [0.0; 256];

    let mut max = 0.0;
    let mut k_star = 0;

    for i in 0..256 {
        let numer = (global_mean_intensity * probabilities_class1[i] - mean_intensities_class1[i])
            .powf(2.0);
        let denom = (probabilities_class1[i]) * (1.0 - probabilities_class1[i]);

        between_class_var[i] = numer / denom;

        if between_class_var[i] > max {
            max = between_class_var[i];
            k_star = i;
        }
    }

    return k_star;
}

pub fn otsu(image: &GrayImage) -> GrayImage {
    let (width, height) = image.dimensions();
    let mut result = GrayImage::new(width, height);

    let otsu_threshold = otsu_threshold(image);

    let dark_pixel = Luma([0]);
    let light_pixel = Luma([255]);

    for (col, row, pixel) in image.enumerate_pixels() {
        if *pixel.channels().get(0).unwrap() as usize > otsu_threshold {
            result.put_pixel(col, row, light_pixel);
        } else {
            result.put_pixel(col, row, dark_pixel);
        }
    }

    result
}
