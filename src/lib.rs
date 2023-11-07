use std::path::{Path, PathBuf};
use crate::mz3::Mz3;
use flate2::{write::GzEncoder, Compression};
use fs_err::File;
use std::io::BufWriter;

mod io;
mod mz3;
mod obj;
mod scalar;

/// Convert MNI `.obj` or `.txt` file to Surf-Ice `.mz3` file.
pub fn mni2mz3<P: AsRef<Path>, B: Into<PathBuf>>(input: P, output: B) -> main_error::MainResult {
    let data = Mz3::read_mni_file(input)?;
    let output_file = File::create(output)?;
    let writer = BufWriter::new(output_file);
    let mut gz = GzEncoder::new(writer, Compression::default());
    data.write_to(&mut gz)?;
    gz.finish()?;
    Ok(())
}
