"""Point cloud rendering example."""

import frustum
import math

# Generate points on a helix
n_points = 500
positions = []
scalars = []

for i in range(n_points):
    t = i / n_points * 4 * math.pi  # 2 full rotations
    x = math.cos(t) * 0.5
    y = t / (4 * math.pi) - 0.5  # -0.5 to 0.5
    z = math.sin(t) * 0.5
    positions.extend([x, y, z])
    scalars.append(i / n_points)  # scalar from 0 to 1

# Create point cloud
pc = frustum.PointCloud(positions=positions, point_size=8.0)
pc_with_scalars = pc.with_scalars(scalars)
pc_with_material = pc_with_scalars.with_material("rainbow")

# Create colormap material
rainbow = frustum.ScalarMappedMaterial("rainbow", "viridis", [0.0, 1.0])

# Create camera and scene
camera = frustum.Camera.perspective([2.0, 1.0, 2.0], [0.0, 0.0, 0.0], fov=45.0)
scene = frustum.Scene(camera, bounds=([-1, -1, -1], [1, 1, 1]))

scene.add_scalar_material(rainbow)
scene.add_points(pc_with_material)

# Render
config = frustum.RenderConfig(width=800, height=600, background=[0.02, 0.02, 0.05, 1.0])
png_data = frustum.render_to_png(scene, config)
frustum.save_png(png_data, "point_cloud.png")

print("Rendered to point_cloud.png")
