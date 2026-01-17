# Frustum

GPU-accelerated scientific 3D visualization for Python.

## Requirements

- Python 3.8+
- A GPU with Vulkan, Metal, or DX12 support
- Rust toolchain (for building from source)

## Installation

```bash
pip install frustum
```

### Building from source

```bash
# Install maturin
pip install maturin

# Clone and build
git clone https://github.com/gcol33/pyfrustum.git
cd pyfrustum
maturin develop --release
```

## Quick Start

```python
import frustum

# Create a camera
camera = frustum.Camera.perspective([5.0, 3.0, 5.0], [0.0, 0.0, 0.0], fov=45.0)

# Create a scene with bounds
scene = frustum.Scene(camera, bounds=([-1, -1, -1], [1, 1, 1]))

# Create a simple triangle mesh
mesh = frustum.Mesh(
    positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
    indices=[0, 1, 2]
)
scene.add_mesh(mesh)

# Render to PNG
config = frustum.RenderConfig(width=800, height=600)
png_data = frustum.render_to_png(scene, config)
frustum.save_png(png_data, "output.png")
```

## Features

### Geometry Primitives

- **Mesh**: Triangle meshes with optional normals and per-vertex scalars
- **PointCloud**: Point sets with configurable size
- **Polyline**: Connected line segments

### Materials

- **SolidMaterial**: Solid colors with optional alpha
- **ScalarMappedMaterial**: Colormaps (viridis, plasma, magma, inferno, cividis)

### Isosurface Extraction

```python
# Create a 3D volume
import numpy as np

nx, ny, nz = 50, 50, 50
x = np.linspace(-1, 1, nx)
y = np.linspace(-1, 1, ny)
z = np.linspace(-1, 1, nz)
X, Y, Z = np.meshgrid(x, y, z, indexing='ij')

# Sphere SDF
values = np.sqrt(X**2 + Y**2 + Z**2) - 0.5

# Create volume and extract isosurface
volume = frustum.Volume(
    values=values.flatten().tolist(),
    dimensions=[nx, ny, nz],
    spacing=[2.0/nx, 2.0/ny, 2.0/nz],
    origin=[-1.0, -1.0, -1.0]
)

mesh = frustum.marching_cubes(volume, iso_value=0.0)
```

### Camera Types

```python
# Perspective camera (default FOV: 45 degrees)
cam = frustum.Camera.perspective([5, 3, 5], [0, 0, 0], fov=45.0)

# Orthographic camera
cam = frustum.Camera.orthographic([0, 0, 10], [0, 0, 0], height=5.0)
```

### Lighting

```python
# Custom directional light
light = frustum.Light([1.0, 1.0, 1.0], intensity=1.0)

# Presets
light = frustum.Light.three_quarter()     # Most versatile
light = frustum.Light.scientific_flat()   # Minimal shadows
light = frustum.Light.studio_soft()       # Soft studio lighting
light = frustum.Light.rim_highlight()     # Edge emphasis
light = frustum.Light.depth_emphasis()    # Depth perception
light = frustum.Light.side_light()        # Side illumination

scene.set_light(light)
```

## API Reference

### Classes

| Class | Description |
|-------|-------------|
| `Camera` | Perspective or orthographic camera |
| `Light` | Directional light source |
| `Mesh` | Triangle mesh geometry |
| `PointCloud` | Point cloud geometry |
| `Polyline` | Connected line segments |
| `SolidMaterial` | Solid color material |
| `ScalarMappedMaterial` | Colormap material |
| `Volume` | 3D scalar field |
| `Scene` | Container for geometry and settings |
| `RenderConfig` | Rendering parameters |

### Functions

| Function | Description |
|----------|-------------|
| `marching_cubes(volume, iso_value)` | Extract isosurface |
| `marching_cubes_multi(volume, iso_values)` | Extract multiple isosurfaces |
| `render_to_png(scene, config)` | Render scene to PNG bytes |
| `save_png(data, path)` | Save PNG bytes to file |

## Troubleshooting

### No GPU adapter found

Frustum requires a GPU with Vulkan, Metal, or DX12 support. On Linux, ensure Vulkan drivers are installed:

```bash
# Ubuntu/Debian
sudo apt install mesa-vulkan-drivers

# Fedora
sudo dnf install mesa-vulkan-drivers
```

### Build errors

Ensure you have the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## License

MIT OR Apache-2.0
