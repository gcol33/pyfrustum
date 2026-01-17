"""Basic tests for frustum package."""

import pytest


def test_import():
    """Test that frustum can be imported."""
    import frustum
    assert hasattr(frustum, 'Camera')
    assert hasattr(frustum, 'Scene')
    assert hasattr(frustum, 'Mesh')


def test_camera_perspective():
    """Test perspective camera creation."""
    import frustum

    cam = frustum.Camera.perspective([5.0, 3.0, 5.0], [0.0, 0.0, 0.0], fov=45.0)
    assert cam is not None
    assert "Camera" in repr(cam)


def test_camera_orthographic():
    """Test orthographic camera creation."""
    import frustum

    cam = frustum.Camera.orthographic([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], height=5.0)
    assert cam is not None


def test_mesh_creation():
    """Test mesh creation."""
    import frustum

    # Simple triangle
    mesh = frustum.Mesh(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
        indices=[0, 1, 2]
    )
    assert mesh.vertex_count == 3
    assert mesh.triangle_count == 1


def test_mesh_with_material():
    """Test mesh with material."""
    import frustum

    mesh = frustum.Mesh(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
        indices=[0, 1, 2]
    )
    mesh_with_mat = mesh.with_material("red")
    assert mesh_with_mat is not None


def test_point_cloud():
    """Test point cloud creation."""
    import frustum

    pc = frustum.PointCloud(
        positions=[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        point_size=5.0
    )
    assert pc.point_count == 2


def test_polyline():
    """Test polyline creation."""
    import frustum

    line = frustum.Polyline(
        positions=[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 0.0, 0.0],
        line_width=2.0
    )
    assert line is not None


def test_solid_material():
    """Test solid material creation."""
    import frustum

    mat = frustum.SolidMaterial("red", [1.0, 0.0, 0.0])
    assert "red" in repr(mat)


def test_scalar_mapped_material():
    """Test scalar-mapped material creation."""
    import frustum

    mat = frustum.ScalarMappedMaterial("temp", "viridis", [0.0, 1.0])
    assert "viridis" in repr(mat)


def test_light():
    """Test light creation."""
    import frustum

    light = frustum.Light([1.0, 1.0, 1.0], intensity=1.0)
    assert light is not None


def test_light_presets():
    """Test light presets."""
    import frustum

    presets = [
        frustum.Light.scientific_flat(),
        frustum.Light.studio_soft(),
        frustum.Light.rim_highlight(),
        frustum.Light.depth_emphasis(),
        frustum.Light.side_light(),
        frustum.Light.three_quarter(),
    ]
    assert all(l is not None for l in presets)


def test_light_zero_direction_error():
    """Test that zero direction raises error."""
    import frustum

    with pytest.raises(ValueError, match="zero"):
        frustum.Light([0.0, 0.0, 0.0], intensity=1.0)


def test_volume():
    """Test volume creation."""
    import frustum

    # 2x2x2 volume
    values = [0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0]
    vol = frustum.Volume(
        values=values,
        dimensions=[2, 2, 2],
        spacing=[1.0, 1.0, 1.0],
        origin=[0.0, 0.0, 0.0]
    )
    assert vol is not None
    vmin, vmax = vol.value_range()
    assert vmin == 0.0
    assert vmax == 1.0


def test_volume_dimension_mismatch():
    """Test volume dimension validation."""
    import frustum

    with pytest.raises(ValueError, match="Expected"):
        frustum.Volume(
            values=[1.0, 2.0, 3.0],  # 3 values
            dimensions=[2, 2, 2],     # expects 8 values
        )


def test_scene_creation():
    """Test scene creation."""
    import frustum

    cam = frustum.Camera.perspective([5.0, 3.0, 5.0], [0.0, 0.0, 0.0])
    scene = frustum.Scene(cam, bounds=([-1, -1, -1], [1, 1, 1]))
    assert scene is not None


def test_scene_add_geometry():
    """Test adding geometry to scene."""
    import frustum

    cam = frustum.Camera.perspective([5.0, 3.0, 5.0], [0.0, 0.0, 0.0])
    scene = frustum.Scene(cam, bounds=([-1, -1, -1], [1, 1, 1]))

    mesh = frustum.Mesh(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
        indices=[0, 1, 2]
    )
    scene.add_mesh(mesh)

    pc = frustum.PointCloud([0.0, 0.0, 0.0], point_size=5.0)
    scene.add_points(pc)

    line = frustum.Polyline([0.0, 0.0, 0.0, 1.0, 1.0, 1.0])
    scene.add_polyline(line)

    assert "meshes=1" in repr(scene)
    assert "points=1" in repr(scene)
    assert "lines=1" in repr(scene)


def test_render_config():
    """Test render config creation."""
    import frustum

    config = frustum.RenderConfig(width=800, height=600)
    assert config.width == 800
    assert config.height == 600


def test_render_config_custom_background():
    """Test render config with custom background."""
    import frustum

    config = frustum.RenderConfig(
        width=512,
        height=512,
        background=[1.0, 1.0, 1.0, 1.0]
    )
    assert config.background == [1.0, 1.0, 1.0, 1.0]
