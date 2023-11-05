use crate::args::get_paths;
use crate::mz3::Mz3;
use flate2::{write::GzEncoder, Compression};
use fs_err::File;
use std::error::Error;
use std::io::BufWriter;

mod args;
mod io;
mod mz3;
mod scalar;

fn main() -> main_error::MainResult {
    let args: Vec<_> = std::env::args_os().collect();
    let (input_path, output_path) = get_paths(args)?;
    let data = Mz3::read_mni_file(input_path)?;

    let output_file = File::create(output_path)?;
    // let writer = BufWriter::new(output_file);
    // let mut gz = GzEncoder::new(writer, Compression::default());
    // data.write_to(&mut gz)?;

    data.write_to(&mut BufWriter::new(output_file))?;

    Ok(())
}
