# mni2mz3

[![crates.io](https://img.shields.io/crates/v/mni2mz3?label=version)](https://crates.io/crates/mni2mz3)
[![MIT License](https://img.shields.io/github/license/fnndsc/mni2mz3)](https://github.com/FNNDSC/mni2mz3/blob/main/LICENSE)
[![test](https://github.com/FNNDSC/mni2mz3/actions/workflows/test.yml/badge.svg)](https://github.com/FNNDSC/mni2mz3/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/FNNDSC/mni2mz3/branch/master/graph/badge.svg?token=1YQRZWW95S)](https://codecov.io/gh/FNNDSC/mni2mz3)

Converts a file from MNI polygonal surface mesh format (`.obj`\*) or
vertex-wise data (e.g. curvature, cortical thickness, `*.txt`)
to [Surf-Ice MZ3 (`.mz3`)](https://github.com/neurolabusc/surf-ice/tree/master/mz3).
Useful for visualizing surfaces using [Surf-Ice](https://github.com/neurolabusc/surf-ice) or [NiiVue](https://github.com/niivue/niivue).

> [!WARNING]
> \*Not to be confused with Wavefront .obj, which is a different spec but with the same file extension.

## Installation

There are many ways to install and use `mni2mz3`. Linux, Mac, and Windows are supported.

### Using _ChRIS_

The easiest way to run `mni2mz3` is on [_ChRIS_](https://chrisproject.org), no installation needed.
Simply upload your data to a feed in https://app.chrisproject.org, then run `pl-mni2common`.

### Using Apptainer

`pl-mni2common` is a _ChRIS_ plugin wrapper for `mni2mz3`, which means you can
use its container image to run `mni2mz3`.

```shell
apptainer run docker://ghcr.io/fnndsc/pl-mni2common:latest mni2mz3 input.obj output.mz3
```

Call the wrapper script `mni2mz3` instead to do bulk processing on an input directory.

```shell
apptainer run docker://ghcr.io/fnndsc/pl-mni2common:latest mni2mz3 inputdir/ outputdir/
```

### Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall mni2mz3
```

### Using pip

```shell
pip install mni2mz3
```

### Manual Download

Select and download the right binary for your architecture and OS from GitHub Releases:
https://github.com/FNNDSC/mni2mz3/releases/latest

### Compile From Source

Install [Rust](https://rustup.rs), then run

```shell
cargo install mni2mz3
```

## Usage

```shell
# convert mesh
mni2mz3 surface_81920.obj surface.mz3

# convert data
mni2mz3 thickness.txt thickness.mz3
```

To do bulk conversions, use the [_ChRIS_ plugin wrapper](#using-chris).

## Details

- Output file will be gzip compressed.
- For surfaces, only triangle meshes are supported.
- For data, only 32-bit single-precision "float" is supported.

## Testing

It is recommended to install [cargo-nextest](https://nexte.st/).

```shell
cargo nextest run
```
