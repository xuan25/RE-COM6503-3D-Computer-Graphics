//! Port of `legacy/scene/animator/utils/Bezier.java`.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bezier {
    pub value: f64,
    pub slope: f64,
}

impl Bezier {
    pub fn new(sequence: &[f64], ratio: f64) -> Self {
        assert!(
            sequence.len() >= 2,
            "Bezier needs at least two control points"
        );
        let x1 = Self::value_at(&sequence[..sequence.len() - 1], ratio);
        let x2 = Self::value_at(&sequence[1..], ratio);
        let slope = x2 - x1;
        Self {
            value: x1 + slope * ratio,
            slope,
        }
    }

    pub fn value_at(sequence: &[f64], ratio: f64) -> f64 {
        let n = sequence.len().saturating_sub(1);
        sequence
            .iter()
            .enumerate()
            .map(|(index, value)| {
                Self::combination(index, n) as f64
                    * value
                    * (1.0 - ratio).powi((n - index) as i32)
                    * ratio.powi(index as i32)
            })
            .sum()
    }

    fn combination(k: usize, n: usize) -> usize {
        Self::factorial(n) / (Self::factorial(k) * Self::factorial(n - k))
    }
    fn factorial(value: usize) -> usize {
        (1..=value).product::<usize>().max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Bezier;
    #[test]
    fn linear_curve_is_linear() {
        assert!((Bezier::value_at(&[2.0, 10.0], 0.25) - 4.0).abs() < f64::EPSILON);
    }
    #[test]
    fn endpoint_values_match_control_points() {
        let points = [0.0, 3.0, -1.0, 8.0];
        assert_eq!(Bezier::new(&points, 0.0).value, 0.0);
        assert_eq!(Bezier::new(&points, 1.0).value, 8.0);
    }
}
