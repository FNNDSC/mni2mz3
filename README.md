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

TODO

## Usage

```shell
# convert mesh
mni2mz3 surface_81920.obj surface.mz3

# convert data
mni2mz3 thickness.txt thickness.mz3
```

## Details

- Output file will be gzip compressed.
- For surfaces, only triangle meshes are supported.
- For data, only 32-bit single-precision "float" is supported.

## Testing

It is recommended to install [cargo-nextest](https://nexte.st/).

```shell
cargo nextest run
```
