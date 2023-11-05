# obj2mz3

Converts a file from MNI polygonal surface mesh format (`.obj`\*) or
vertex-wise data (e.g. curvature, cortical thickness, `*.txt`)
to [Surf-Ice MZ3 (`.mz3`)](https://github.com/neurolabusc/surf-ice/tree/master/mz3).
Useful for visualizing surfaces using [Surf-Ice](https://github.com/neurolabusc/surf-ice) or [NiiVue](https://github.com/niivue/niivue).

\*Not to be confused with Wavefront .obj, which is a different spec but with the same file extension.

## Installation

TODO

## Usage

```shell
# convert mesh
mni2mz3 surface_81920.obj surface.mz3

# convert data
mni2mz3 thickness.txt thickness.txt
```

## Details

- TODO Output file will be gzip compressed.
- For surfaces, only triangle meshes are supported.
- For data, only 32-bit single-precision "float" (`isSCALAR`) is supported.
  (`isDOUBLE` is not supported)
