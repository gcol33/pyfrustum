"""Marching cubes isosurface extraction example."""

import frustum

# Create a 3D volume with a sphere SDF
# Using pure Python (numpy optional)
nx, ny, nz = 50, 50, 50
spacing = [2.0 / nx, 2.0 / ny, 2.0 / nz]
origin = [-1.0, -1.0, -1.0]

# Generate sphere SDF: distance to surface of sphere with radius 0.6
values = []
for iz in range(nz):
    for iy in range(ny):
        for ix in range(nx):
            x = origin[0] + (ix + 0.5) * spacing[0]
            y = origin[1] + (iy + 0.5) * spacing[1]
            z = origin[2] + (iz + 0.5) * spacing[2]
            # SDF: positive outside, negative inside
            dist = (x*x + y*y + z*z) ** 0.5 - 0.6
            values.append(dist)

# Create volume
volume = frustum.Volume(
    values=values,
    dimensions=[nx, ny, nz],
    spacing=spacing,
    origin=origin
)

print(f"Volume: {volume}")
print(f"Value range: {volume.value_range()}")

# Extract isosurface at the zero level
mesh = frustum.marching_cubes(volume, iso_value=0.0)
print(f"Mesh: {mesh}")

# Create camera and scene
camera = frustum.Camera.perspective([2.0, 1.5, 2.0], [0.0, 0.0, 0.0], fov=45.0)
scene = frustum.Scene(camera, bounds=([-1, -1, -1], [1, 1, 1]))

# Add scalar-mapped material using vertex z-coordinate as scalar
# The mesh already has gradient-based normals from marching cubes
mat = frustum.SolidMaterial("surface", [0.3, 0.6, 0.9])
scene.add_solid_material(mat)

mesh_with_material = mesh.with_material("surface")
scene.add_mesh(mesh_with_material)

# Set lighting for good depth perception
light = frustum.Light.three_quarter()
scene.set_light(light)

# Render
config = frustum.RenderConfig(width=800, height=800, background=[0.05, 0.05, 0.1, 1.0])
png_data = frustum.render_to_png(scene, config)
frustum.save_png(png_data, "sphere_isosurface.png")

print("Rendered to sphere_isosurface.png")
