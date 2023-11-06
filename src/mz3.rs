//! MZ3 file format.
//! https://github.com/neurolabusc/surf-ice/tree/master/mz3

use crate::obj::*;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

const MZ3_MAGIC_SIGNATURE: &[u8; 2] = &[0x4D, 0x5A];

const MZ3_IS_SCALAR: &[u8; 2] = &[0x08, 0x00];

const MZ3_IS_FACE_AND_IS_VERT: &[u8; 2] = &[0x03, 0x00];

const NSKIP_EMPTY: &[u8; 4] = &[0x00, 0x00, 0x00, 0x00];

/// MZ3 file representation.
pub(crate) enum Mz3 {
    /// Triangle mesh
    Mesh { faces: Vec<u32>, vertices: Vec<f32> },
    /// Scalar vertex-wise data
    Scalar(Vec<f32>),
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum LoadFileError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Bad value in {0}: {1}")]
    BadValue(PathBuf, String),

    #[error("Unsupported file extension \"{0}\"")]
    Unsupported(String),

    #[error(transparent)]
    MniMeshError(#[from] MniMeshParseError),
}

impl Mz3 {
    /// Write binary data
    pub fn write_to<W: Write>(self, sink: &mut W) -> io::Result<()> {
        match self {
            Mz3::Mesh { faces, vertices } => mesh_to_mz3(faces, vertices, sink),
            Mz3::Scalar(data) => data_to_mz3(data, sink),
        }
    }

    /// Read an MNI `.obj` surface or `.txt` values file.
    pub fn read_mni_file<P: AsRef<Path>>(p: P) -> Result<Self, LoadFileError> {
        let path = p.as_ref();
        match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
            "obj" => {
                let string_data = crate::io::read_file(path)?;
                MniMeshRaw::parse_obj(string_data.iter())
                    .map_err(LoadFileError::MniMeshError)
                    .map(|m| m.into())
            }
            "txt" => {
                let string_data = crate::io::read_file(path)?;
                crate::scalar::parse_floats(string_data.as_slice())
                    .map(Mz3::Scalar)
                    .map_err(|e| LoadFileError::BadValue(path.to_path_buf(), e.to_string()))
            }
            other => Err(LoadFileError::Unsupported(other.to_string())),
        }
    }
}

impl From<MniMeshRaw> for Mz3 {
    fn from(obj: MniMeshRaw) -> Self {
        Mz3::Mesh {
            faces: obj.indices,
            vertices: obj.point_array,
        }
    }
}

fn data_to_mz3<W: Write>(data: Vec<f32>, sink: &mut W) -> io::Result<()> {
    sink.write_all(MZ3_MAGIC_SIGNATURE)?;
    sink.write_all(MZ3_IS_SCALAR)?;
    sink.write_all(&[0, 0, 0, 0, 0, 0, 0, 0])?; // NFACE=0 NVERT=0
    sink.write_all(NSKIP_EMPTY)?;
    for element in data {
        sink.write_all(&element.to_le_bytes())?;
    }
    Ok(())
}

fn mesh_to_mz3<W: Write>(faces: Vec<u32>, vertices: Vec<f32>, sink: &mut W) -> io::Result<()> {
    let n_face: u32 = (faces.len() / 3) as u32;
    let n_vert: u32 = (vertices.len() / 3) as u32;
    sink.write_all(MZ3_MAGIC_SIGNATURE)?;
    sink.write_all(MZ3_IS_FACE_AND_IS_VERT)?;
    sink.write_all(&n_face.to_le_bytes())?;
    sink.write_all(&n_vert.to_le_bytes())?;
    sink.write_all(NSKIP_EMPTY)?;

    // do I need to worry about winding order?
    for face_element in faces {
        sink.write_all(&face_element.to_le_bytes())?;
    }

    for vertex_element in vertices {
        sink.write_all(&vertex_element.to_le_bytes())?;
    }

    Ok(())
}
