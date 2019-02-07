const MAX_ITERATION: usize = 1000;

fn is_mandelbrot(constant: (f32, f32)) -> Option<usize> {
    let mut value = constant;
    for i in 0..MAX_ITERATION {
        if value.0 > 2.0 || value.1 > 2.0 {
            return Some(i);
        }
        value = (
            value.0 * value.0 + constant.0,
            value.1 * value.1 + constant.1,
        );
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_mandelbrot() {
        let values = vec![(2.0, 0.0), (0.0, 2.0), (1.0, 3.0), (1.0, -2.0)];

        for value in values.into_iter() {
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
            match is_mandelbrot(value) {
                None => (),
                Some(_c) => panic!("{:?}", value),
            }
        }
    }

}
