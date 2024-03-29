use std::collections::hash_map::IterMut;

use image::{self, GenericImageView, GrayImage, Pixel};

struct GrayHistogram {
    histogram: [u32; 256],
    probabilities: [f32; 256],
}

fn make_grayhistrogram(image: &GrayImage) -> GrayHistogram {
    let mut histogram = [0; 256];
    let mut probabilities = [0.0; 256];

    let (width, height) = image.dimensions();
    let sum = width * height;

    for pixel in image.pixels() {
        histogram[*pixel.channels().get(0).unwrap() as usize] += 1;
    }

    //Need to fix this
    probabilities
        .iter_mut()
        .enumerate()
        .map(|(i, x)| *x + (histogram[i] / sum) as f32);

    println!("histogram: {:?}", histogram);
    println!("probabilities: {:?}", probabilities);

    GrayHistogram {
        histogram,
        probabilities,
    }
}

pub fn otsu_threshold(image: &GrayImage) -> f32 {
    let easy_histogram = make_grayhistrogram(image);

    //q1(k)
    let mut probabilities_class1 = [0.0; 256];

    //m1(k)
    let mut mean_intensities_class1 = [0.0; 256];

    for i in 0..easy_histogram.histogram.len() {
        let mut sum = 0.0;
        let mut sum_probabilities = 0.0;

        for j in 0..i {
            sum += easy_histogram.probabilities[j];
            sum_probabilities += sum * i as f32;
        }

        probabilities_class1[i] = sum;
        mean_intensities_class1[i] = sum_probabilities;
    }

    //mg
    let global_mean_intensity = *mean_intensities_class1.last().unwrap();

    //sigmab^2
    let mut between_class_var = [0.0; 256];

    for i in 0..easy_histogram.histogram.len() {
        between_class_var[i] =
            (global_mean_intensity * probabilities_class1[i] - mean_intensities_class1[i]).powf(2.0)
                / ((probabilities_class1[i]) * (1.0 - probabilities_class1[i]))
    }

    // println!("q1: {:?}", probabilities_class1);
    // println!("m1: {:?}", mean_intensities_class1);
    // println!("mq: {:?}", global_mean_intensity);
    // println!("sigmab: {:?}", between_class_var);

    //return max(between_class_var);
    //return between_class_var.iter().max().unwrap;
    let max = between_class_var.iter().fold(f32::MIN, |a, &b| a.max(b));
    return max;
}
