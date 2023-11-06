//! (Incomplete) parser for the MNI obj surface mesh format.
//!
//! https://www.bic.mni.mcgill.ca/users/mishkin/mni_obj_format.pdf

/// MNI polygonal mesh data.
///
/// - Only polygonal mesh is supported.
/// - All polygons must be triangles.
/// - Color is not supported.
///
/// Data are as-is in the file representation (not an ergonomic API).
pub(crate) struct MniMeshRaw {
    // pub surfprop: SurfProp,
    pub point_array: Vec<f32>,
    // pub normals: Vec<(f32, f32, f32)>,
    // color not supported
    pub indices: Vec<u32>,
}

// /// Surface property flags.
// pub(crate) struct SurfProp {
//     pub ambient_colour: f32,
//     pub diffuse_reflectivity: f32,
//     pub specular_reflectance: f32,
//     pub speculate_scattering_exponent: u8,
//     pub transparency: f32,
// }

#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum MniMeshParseError {
    #[error("Unsupported mesh feature: {0}")]
    Unsupported(String),

    #[error("File is empty")]
    EmptyFile,

    #[error("Missing SurfProp")]
    MissingSurfProp,

    // #[error("Invalid mesh: {0}")]
    // Invalid(String)
    #[error("Cannot read n_points")]
    CannotReadNPoints,

    #[error("Number of points found {n_found} is fewer than n_points={n_points}")]
    TooFewPoints { n_points: usize, n_found: usize },

    #[error("Point data at index {0} is not a float")]
    InvalidPoint(usize),

    #[error("Cannot parse nitems (number of polygons appearing after the normal vectors)")]
    InvalidNItems,

    #[error("Missing colour flag")]
    MissingColourFlag,

    #[error("Missing end indices")]
    MissingEndIndices { nitems: usize, n_found: usize },

    #[error("First element of end indices must be 3")]
    InvalidFirstEndIndex,

    #[error("Invalid end index at {0} (end indices array must be a sequence of natural numbers counting up by 3, i.e. only triangular meshes are supported)")]
    InvalidEndIndices(usize),

    #[error("Member of indices array at {0} cannot be parsed as float")]
    InvalidIndex(usize),

    #[error("Expected indices array to have length {expected} (greatest value in end_indices) however found {actual} values")]
    InvalidIndicesLength { expected: usize, actual: usize },

    #[error("Colour table is shorter than expected: expected {expected} elements, found {actual}")]
    MissingColourTable { expected: usize, actual: usize },

    #[error("Expected to find {n_vectors} normal vectors, but only found {n_found}")]
    TooFewNormals { n_vectors: usize, n_found: usize },
}

impl MniMeshRaw {
    /// Parse a polygon object in the MNI obj format from whitespace-delimited strings.
    ///
    /// RiiR: https://github.com/FNNDSC/pybicpl/blob/29a21af357a19708ea170c8bf2b8b42e12b3cab4/bicpl/obj.py#L143-L193
    pub fn parse_obj<S: AsRef<str>>(
        mut data: impl Iterator<Item = S>,
    ) -> Result<Self, MniMeshParseError> {
        consume_header(&mut data)?;
        let n_points: usize = data
            .next()
            .and_then(|s| s.as_ref().parse().ok())
            .ok_or(MniMeshParseError::CannotReadNPoints)?;

        let point_array = (0..(n_points * 3))
            .map(|i| {
                data.next()
                    .ok_or(MniMeshParseError::TooFewPoints {
                        n_points,
                        n_found: i + 1,
                    })
                    .and_then(|s| {
                        s.as_ref()
                            .parse()
                            .map_err(|_| MniMeshParseError::InvalidPoint(i))
                    })
            })
            .collect::<Result<_, MniMeshParseError>>()?;

        // count normal vectors, but don't save them
        let n_normal_vector_components = (0..(n_points * 3))
            .map(|_| data.next())
            .filter(|e| e.is_some())
            .count();
        if n_normal_vector_components != (n_points * 3) {
            let n_vectors = n_points / 3;
            let n_found = n_normal_vector_components / 3;
            return Err(MniMeshParseError::TooFewNormals { n_vectors, n_found });
        }

        // Number of polygons defined.
        let nitems: usize = data
            .next()
            .ok_or(MniMeshParseError::InvalidNItems)
            .and_then(|s| {
                s.as_ref()
                    .parse()
                    .map_err(|_| MniMeshParseError::InvalidNItems)
            })?;

        let ele = data.next().ok_or(MniMeshParseError::MissingColourFlag)?;
        let colour_flag = ele.as_ref();
        if colour_flag != "0" {
            let msg =
                format!("Unsupported colour flag: {colour_flag} (only the value 0 is supported)");
            return Err(MniMeshParseError::Unsupported(msg));
        }
        // consume the RGBA volues, make sure they're there, then discard the values
        let colour_table_length = (0..4).map(|_| data.next()).filter(|o| o.is_some()).count();
        if colour_table_length != 4 {
            return Err(MniMeshParseError::MissingColourTable {
                expected: 4,
                actual: colour_table_length,
            });
        }

        let last_end_index = consume_end_indices(&mut data, nitems)?;

        // everything else are indices
        let indices = data
            .map(|s| s.as_ref().parse())
            .enumerate()
            .map(|(i, r)| r.map_err(|_| MniMeshParseError::InvalidIndex(i)))
            .collect::<Result<Vec<_>, MniMeshParseError>>()?;

        // I think the documentation is wrong here. Documentation says
        //
        // > The length of this array must be equal to the greatest value in the end indices array plus one
        //
        // The "plus one" part doesn't seem to be correct
        if indices.len() != last_end_index {
            return Err(MniMeshParseError::InvalidIndicesLength {
                expected: last_end_index,
                actual: indices.len(),
            });
        }

        let ret = Self {
            point_array,
            indices,
        };
        Ok(ret)
    }
}

/// Consume the object class character and surfprop.
fn consume_header<S: AsRef<str>>(
    data: &mut impl Iterator<Item = S>,
) -> Result<(), MniMeshParseError> {
    let element = data.next().ok_or(MniMeshParseError::EmptyFile)?;
    let object_class = element.as_ref();
    if object_class != "P" {
        let msg =
            format!("Unsupported object class: {object_class} (First token of file must be \"P\")");
        return Err(MniMeshParseError::Unsupported(msg));
    }

    // skip over surfprop
    let has_surfprop = (0..5).map(|_| data.next()).all(|o| o.is_some());
    if has_surfprop {
        Ok(())
    } else {
        Err(MniMeshParseError::MissingSurfProp)
    }
}

/// Consume the `end_indices` array and verify that `end_indices` is a sequence
/// which counts up by 3, so 3, 6, 9, 12, 15, ...
///
/// Returns the last end index (which is also the greatest value).
fn consume_end_indices<S: AsRef<str>>(
    data: &mut impl Iterator<Item = S>,
    nitems: usize,
) -> Result<usize, MniMeshParseError> {
    // get the first element from end_indices
    let first_end_index = data
        .next()
        .ok_or(MniMeshParseError::MissingEndIndices { nitems, n_found: 0 })?;
    if first_end_index.as_ref() != "3" {
        return Err(MniMeshParseError::InvalidFirstEndIndex);
    }
    (0..(nitems - 1))
        .map(|i| {
            let i = i + 1;
            // consume the rest of end_indices from the iterator
            data.next()
                .ok_or(MniMeshParseError::MissingEndIndices { nitems, n_found: i })
                .map(|s| (i, s))
        })
        .try_fold((0, 3), |(_prev_i, prev), next_result| {
            // use accumulator to compare previous value to next
            next_result.and_then(|(i, next_ele)| {
                if let Ok(next) = next_ele.as_ref().parse() {
                    // check sequence is counting up by 3
                    if prev + 3 == next {
                        Ok((i, next))
                    } else {
                        // does not count up by 3
                        Err(MniMeshParseError::InvalidEndIndices(i))
                    }
                } else {
                    // failed to parse as int
                    Err(MniMeshParseError::InvalidEndIndices(i))
                }
            })
        })
        .map(|(_i, last)| last)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_consume_end_indices_works() {
        let input = &["3", "6", "9", "12", "leftover0", "leftover1"];
        let nitems = 4;
        let mut data = input.iter();
        assert_eq!(consume_end_indices(&mut data, nitems), Ok(12));
        assert_eq!(data.count(), 2);
    }

    #[rstest]
    fn test_consume_end_indices_bad_start() {
        let input = &["2", "6", "9", "12", "leftover0", "leftover1"];
        let nitems = 4;
        let mut data = input.iter();
        assert_eq!(
            consume_end_indices(&mut data, nitems),
            Err(MniMeshParseError::InvalidFirstEndIndex)
        )
    }

    #[rstest]
    #[case(&["3", "5", "9", "12", "leftover0", "leftover1"], 4, 1)]
    #[case(&["3", "6", "8", "12", "leftover0", "leftover1"], 4, 2)]
    #[case(&["3", "6", "10", "12", "leftover0", "leftover1"], 4, 2)]
    fn test_consume_end_indices_invalid_at(
        #[case] input: &[&'static str],
        #[case] nitems: usize,
        #[case] invalid_index: usize,
    ) {
        let mut data = input.iter();
        assert_eq!(
            consume_end_indices(&mut data, nitems),
            Err(MniMeshParseError::InvalidEndIndices(invalid_index))
        )
    }
}
