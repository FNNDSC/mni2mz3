mod args;

use mni2mz3::mni2mz3;
use crate::args::get_paths;

fn main() -> main_error::MainResult {
    let args: Vec<_> = std::env::args_os().collect();
    let (input, output) = get_paths(args)?;
    mni2mz3(input, output)
}
