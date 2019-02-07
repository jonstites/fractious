use std::ops::{Add, Mul};

const MAX_ITERATION: usize = 1000;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    r: f32,
    i: f32,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            r: self.r * other.r,
            i: self.i * other.i,
        }
    }
}

fn is_mandelbrot(constant: Complex) -> Option<usize> {
    let mut value = constant;
    for i in 0..MAX_ITERATION {
        if value.r > 2.0 || value.i > 2.0 {
            return Some(i);
        }

        value = value * value + constant;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_add() {
        let c1 = Complex { r: 10.0, i: -1.0 };
        let c2 = Complex { r: -2.0, i: 1.0 };
        let result = c1 + c2;
        let expected = Complex { r: 8.0, i: 0.0 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_mul() {
        let c1 = Complex { r: 10.0, i: -1.0 };
        let c2 = Complex { r: -2.0, i: 1.0 };
        let result = c1 * c2;
        let expected = Complex {
            r: 10.0 * -2.0,
            i: -1.0 * 1.0,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_not_mandelbrot() {
        let values = vec![(2.0, 0.0), (0.0, 2.0), (1.0, 3.0), (1.0, -2.0)];

        for value in values.into_iter() {
            let value = Complex {
                r: value.0,
                i: value.1,
            };
            match is_mandelbrot(value) {
                None => panic!(),
                Some(_) => (),
            }
        }
    }

    #[test]
    fn test_is_mandelbrot() {
        let values = vec![(0.0, 0.0), (0.0, 0.1), (-0.03, 0.03)];

        for value in values.into_iter() {
            let value = Complex {
                r: value.0,
                i: value.1,
            };
            match is_mandelbrot(value) {
                None => (),
                Some(_c) => panic!("{:?}", value),
            }
        }
    }

}
