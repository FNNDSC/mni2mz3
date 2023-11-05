//! Scalar vertex-wise data parsing.

pub(crate) fn parse_floats<S: AsRef<str>>(data: &[S]) -> Result<Vec<f32>, ParseFloatError> {
    data.iter()
        .enumerate()
        .map(|(i, n)| n.as_ref().parse().map_err(|_| ParseFloatError(i)))
        .collect()
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
#[error("Could not parse {0}th value as float")]
pub(crate) struct ParseFloatError(usize);

#[cfg(test)]
mod test {

    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::*;

    #[rstest]
    #[case(
        &["0", "1", "1", "1"],
        vec![0.0, 1.0, 1.0, 1.0]
    )]
    #[case(
        &["1.5", "2.6", "3.7", "4.8", "5.9"],
        vec![1.5, 2.6, 3.7, 4.8, 5.9]
    )]
    #[case(
        &["1.5", "-50.5", "-100.2"],
        vec![1.5, -50.5, -100.2]
    )]
    fn test_parse_float_works(#[case] input: &[&str], #[case] expected: Vec<f32>) {
        let actual = parse_floats(input).unwrap();
        for (a, b) in actual.into_iter().zip(expected.into_iter()) {
            assert_abs_diff_eq!(a, b);
        }
    }
}
