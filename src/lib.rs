extern crate num;

use num::Complex;

fn pixel_to_point(
    pixel: (usize, usize),
    pixel_dimensions: (usize, usize),
    region: (Complex<f64>, Complex<f64>),
) -> Complex<f64> {
    assert!(pixel.0 < pixel_dimensions.0);
    assert!(pixel.1 < pixel_dimensions.1);

    let (upper_left, lower_right) = region;

    let re_step_size = (lower_right.re - upper_left.re) / pixel_dimensions.0 as f64;
    let im_step_size = (lower_right.im - upper_left.im) / pixel_dimensions.1 as f64;

    // Add half a pixel, so we're sampling from the middle of the pixel, not the corner
    let re = (pixel.0 as f64 + 0.5) * re_step_size + upper_left.re;
    let im = (pixel.1 as f64 + 0.5) * im_step_size + upper_left.im;

    Complex { re, im }
}

fn escape_time_algorithm(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.re > 2.0 || z.im > 2.0 {
            return Some(i);
        }
    }

    None
}

pub fn escape_times_region(
    pixel_dimensions: (usize, usize),
    region: (Complex<f64>, Complex<f64>),
    limit: u32,
) -> Vec<Option<u32>> {
    let mut buffer = vec![None; pixel_dimensions.0 * pixel_dimensions.1];

    for (i, buf_value) in buffer.iter_mut().enumerate() {
        let pixel = (i % pixel_dimensions.0, i / pixel_dimensions.0);
        let point = pixel_to_point(pixel, pixel_dimensions, region);
        *buf_value = escape_time_algorithm(point, limit);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_point() {
        let upper_left = Complex { re: 0.0, im: 1.0 };
        let lower_right = Complex { re: 1.0, im: -1.0 };
        let region = (upper_left, lower_right);
        let pixel_dimensions = (20, 10);

        let pixels = vec![(0, 0), (0, 9), (19, 0), (19, 9)];

        let expected = vec![
            Complex { re: 0.025, im: 0.9 },
            Complex {
                re: 0.025,
                im: -0.9,
            },
            Complex { re: 0.975, im: 0.9 },
            Complex {
                re: 0.975,
                im: -0.9,
            },
        ];

        for (pixel, expected) in pixels.into_iter().zip(expected.into_iter()) {
            let result = pixel_to_point(pixel, pixel_dimensions, region);
            println!("{:?}", result);
            println!("{:?}", expected);
            println!("{:?}", (expected - result).norm());
            assert!((expected - result).norm() < 0.001);
        }
    }

    #[test]
    fn test_escape_time_algorithm_some() {
        let values = vec![(2.0, 0.0), (0.0, 2.0), (1.0, 3.0), (1.0, -2.0)];
        let limit = 100;
        for (re, im) in values.into_iter() {
            let value = Complex { re, im };
            assert!(escape_time_algorithm(value, limit).is_some());
        }
    }

    #[test]
    fn test_escape_time_algorithm_none() {
        let values = vec![(0.0, 0.0), (0.0, 0.1), (-0.03, 0.03)];
        let limit = 100;
        for (re, im) in values.into_iter() {
            let value = Complex { re, im };
            assert!(escape_time_algorithm(value, limit).is_none());
        }
    }

}
