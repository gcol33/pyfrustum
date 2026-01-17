# Frustum

GPU-accelerated scientific 3D visualization for Python.

## Installation

```bash
pip install frustum
```

## Quick Start

```python
import frustum as fr

# Create a camera
camera = fr.Camera.perspective([2.0, 1.5, 2.0], [0.0, 0.0, 0.0], 45.0)

# Create a scene
scene = fr.Scene(camera, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0])

# Add a mesh (e.g., from marching cubes)
volume = fr.Volume(values, [nx, ny, nz], [dx, dy, dz], [ox, oy, oz])
mesh = fr.marching_cubes(volume, iso_value=0.0)
scene.add_mesh(mesh)

# Render to PNG
png_data = fr.render_to_png(scene, 800, 600)
fr.save_png(png_data, "output.png")
```

## Features

- GPU-accelerated rendering via wgpu
- Isosurface extraction (marching cubes)
- Point clouds, polylines, and meshes
- Multiple lighting presets
- Deterministic, publication-quality output
