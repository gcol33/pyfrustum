"""Simple triangle mesh rendering example."""

import frustum

# Create a perspective camera
camera = frustum.Camera.perspective(
    position=[3.0, 2.0, 3.0],
    target=[0.0, 0.0, 0.0],
    fov=45.0
)

# Create a scene with bounds
scene = frustum.Scene(camera, bounds=([-1, -1, -1], [1, 1, 1]))

# Create a simple triangle
mesh = frustum.Mesh(
    positions=[
        -0.5, -0.5, 0.0,   # vertex 0
         0.5, -0.5, 0.0,   # vertex 1
         0.0,  0.5, 0.0,   # vertex 2
    ],
    indices=[0, 1, 2]
)

# Add red material
red = frustum.SolidMaterial("red", [1.0, 0.2, 0.2])
scene.add_solid_material(red)

# Assign material to mesh and add to scene
mesh_with_material = mesh.with_material("red")
scene.add_mesh(mesh_with_material)

# Set lighting
light = frustum.Light.three_quarter()
scene.set_light(light)

# Render
config = frustum.RenderConfig(width=800, height=600)
png_data = frustum.render_to_png(scene, config)
frustum.save_png(png_data, "simple_mesh.png")

print("Rendered to simple_mesh.png")
