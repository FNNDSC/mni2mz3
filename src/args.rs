//! CLI argument parser helper function.

use std::ffi::OsString;
use std::path::PathBuf;

/// Parse the first two positional command-line arguments as paths.
pub(crate) fn get_paths(
    args: impl IntoIterator<Item = OsString>,
) -> Result<(PathBuf, PathBuf), ArgsError> {
    let mut iter = args.into_iter();
    let prog_name = iter.next().unwrap();
    let (input_path, output_path) = iter
        .next()
        .and_then(|arg1| iter.next().map(|arg2| (arg1, arg2)))
        .ok_or_else(|| ArgsError::TooFew(prog_name.to_string_lossy().to_string()))?;
    if iter.next().is_some() {
        Err(ArgsError::TooMany)
    } else {
        Ok((PathBuf::from(input_path), PathBuf::from(output_path)))
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub(crate) enum ArgsError {
    #[error("usage: {0} input_path output_path")]
    TooFew(String),
    #[error("too many arguments")]
    TooMany,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&["mni2mz3"], Err(ArgsError::TooFew("mni2mz3".to_string())))]
    #[case(&["mni2mz3", "one"], Err(ArgsError::TooFew("mni2mz3".to_string())))]
    #[case(&["mni2mz3", "one", "two", "three"], Err(ArgsError::TooMany))]
    #[case(
        &["mni2mz3", "one", "two"],
        Ok((PathBuf::from("one"), PathBuf::from("two")))
    )]
    fn test_get_paths(
        #[case] input: &[&str],
        #[case] expected: Result<(PathBuf, PathBuf), ArgsError>,
    ) {
        let v = input.iter().map(OsString::from).collect::<Vec<_>>();
        assert_eq!(get_paths(v), expected);
    }
}
