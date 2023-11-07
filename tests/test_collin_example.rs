//! Downloads a subset of the CIVET Colin atlas (if necessary) and runs `mni2mz3` on them.
//!
//! This checks that `mni2mz3` can properly read MNI `.obj` and `.txt` files.
//! However, `.mz3` output is **not** validated.

use rstest::*;
use std::path::Path;
use std::process::Command;

const EXAMPLES: &[(&str, &str); 4] = &[
    ("./.my_test_data/surface_left.obj", "https://github.com/aces/CIVET/raw/9818b3cbe8308249e7c373ef1a2a53956512143e/models/colin/colin_white_mc_left.obj"),
    ("./.my_test_data/surface_right.obj", "https://github.com/aces/CIVET/raw/9818b3cbe8308249e7c373ef1a2a53956512143e/models/colin/colin_white_mc_right.obj"),
    ("./.my_test_data/lobes_left.txt", "https://github.com/aces/CIVET/raw/9818b3cbe8308249e7c373ef1a2a53956512143e/models/colin/lobes/surface_atlas_colin_mc_left.txt"),
    ("./.my_test_data/lobes_right.txt", "https://github.com/aces/CIVET/raw/9818b3cbe8308249e7c373ef1a2a53956512143e/models/colin/lobes/surface_atlas_colin_mc_right.txt")
];

#[fixture]
#[once]
fn surface_file_paths() -> Vec<&'static Path> {
    EXAMPLES.iter().map(|(p, url)| {
        let path = Path::new(p);
        path.parent().and_then(|d| std::fs::create_dir_all(d).ok()).unwrap();
        if !path.is_file() {
            let rc = Command::new("curl")
                .arg("-Lsfo")
                .arg(path)
                .arg(url)
                .status()
                .unwrap();
            if !rc.success() {
                panic!("Failed to download {url}");
            }

        }
        path
    }).collect()
}

#[rstest]
fn test_examples(surface_file_paths: &[&'static Path]) {
    let tmp_dir = tempfile::tempdir().unwrap();
    for input_file in surface_file_paths {
        let output_path = tmp_dir.path().join(input_file.file_name().unwrap());
        mni2mz3::mni2mz3(input_file, output_path).unwrap();
    }
}
