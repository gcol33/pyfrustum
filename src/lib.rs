//! Python bindings for Frustum GPU rendering framework.
//!
//! Provides a Pythonic API for creating scientific 3D visualizations.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

// Re-export core types
use frustum_core::{
    Camera as RustCamera,
    Light as RustLight,
    Material as RustMaterial,
    Mesh as RustMesh,
    PointCloud as RustPointCloud,
    Polyline as RustPolyline,
    ScalarMappedMaterial as RustScalarMappedMaterial,
    SolidMaterial as RustSolidMaterial,
    Volume as RustVolume,
    marching_cubes as rust_marching_cubes,
    marching_cubes_multi as rust_marching_cubes_multi,
};
use frustum_core::scene::{Bounds as RustBounds, Scene as RustScene};
use frustum_render::{render_to_png as rust_render_to_png, RenderConfig as RustRenderConfig};

// =============================================================================
// Camera
// =============================================================================

/// Camera for viewing the scene.
///
/// Examples:
///     # Perspective camera looking at origin
///     cam = Camera.perspective([5, 3, 5], [0, 0, 0], fov=45)
///
///     # Orthographic camera
///     cam = Camera.orthographic([0, 0, 10], [0, 0, 0], height=5)
#[pyclass]
#[derive(Clone)]
pub struct Camera {
    inner: RustCamera,
}

#[pymethods]
impl Camera {
    /// Create a perspective camera.
    ///
    /// Args:
    ///     position: Camera position [x, y, z]
    ///     target: Look-at target [x, y, z]
    ///     fov: Field of view in degrees (default: 45)
    #[staticmethod]
    #[pyo3(signature = (position, target, fov=45.0))]
    fn perspective(position: [f32; 3], target: [f32; 3], fov: f32) -> Self {
        Self {
            inner: RustCamera::perspective(position, target, fov),
        }
    }

    /// Create an orthographic camera.
    ///
    /// Args:
    ///     position: Camera position [x, y, z]
    ///     target: Look-at target [x, y, z]
    ///     height: View height in world units (default: 2.0)
    #[staticmethod]
    #[pyo3(signature = (position, target, height=2.0))]
    fn orthographic(position: [f32; 3], target: [f32; 3], height: f32) -> Self {
        Self {
            inner: RustCamera::orthographic(position, target, height),
        }
    }

    fn __repr__(&self) -> String {
        format!("Camera(position={:?}, target={:?})",
            self.inner.position, self.inner.target)
    }
}

// =============================================================================
// Light
// =============================================================================

/// Directional light for Lambertian shading.
///
/// Examples:
///     light = Light([1, 1, 1], intensity=1.0)
///     light = Light.three_quarter()  # Preset
#[pyclass]
#[derive(Clone)]
pub struct Light {
    inner: RustLight,
}

#[pymethods]
impl Light {
    /// Create a directional light.
    ///
    /// Args:
    ///     direction: Direction toward light source [x, y, z]
    ///     intensity: Light intensity (default: 1.0)
    #[new]
    #[pyo3(signature = (direction, intensity=1.0))]
    fn new(direction: [f32; 3], intensity: f32) -> PyResult<Self> {
        if direction.iter().all(|&v| v == 0.0) {
            return Err(PyValueError::new_err("Light direction cannot be zero"));
        }
        Ok(Self {
            inner: RustLight::new(direction, intensity),
        })
    }

    /// Scientific flat lighting preset.
    #[staticmethod]
    fn scientific_flat() -> Self {
        Self { inner: RustLight::scientific_flat() }
    }

    /// Studio soft lighting preset.
    #[staticmethod]
    fn studio_soft() -> Self {
        Self { inner: RustLight::studio_soft() }
    }

    /// Rim highlight lighting preset.
    #[staticmethod]
    fn rim_highlight() -> Self {
        Self { inner: RustLight::rim_highlight() }
    }

    /// Depth emphasis lighting preset.
    #[staticmethod]
    fn depth_emphasis() -> Self {
        Self { inner: RustLight::depth_emphasis() }
    }

    /// Side lighting preset.
    #[staticmethod]
    fn side_light() -> Self {
        Self { inner: RustLight::side_light() }
    }

    /// Three-quarter view lighting preset (most versatile).
    #[staticmethod]
    fn three_quarter() -> Self {
        Self { inner: RustLight::three_quarter() }
    }

    fn __repr__(&self) -> String {
        format!("Light(direction={:?}, intensity={})",
            self.inner.direction, self.inner.intensity)
    }
}

// =============================================================================
// Mesh
// =============================================================================

/// Triangle mesh geometry.
///
/// Examples:
///     mesh = Mesh(positions=[0,0,0, 1,0,0, 0.5,1,0], indices=[0,1,2])
///     mesh = mesh.with_material("red")
#[pyclass]
#[derive(Clone)]
pub struct Mesh {
    inner: RustMesh,
}

#[pymethods]
impl Mesh {
    /// Create a mesh from positions and triangle indices.
    ///
    /// Args:
    ///     positions: Flat list of vertex positions [x0,y0,z0, x1,y1,z1, ...]
    ///     indices: Flat list of triangle indices [i0,i1,i2, ...]
    #[new]
    fn new(positions: Vec<f32>, indices: Vec<u32>) -> Self {
        Self {
            inner: RustMesh::new(positions, indices),
        }
    }

    /// Set the material ID for this mesh.
    fn with_material(&self, material_id: &str) -> Self {
        Self {
            inner: self.inner.clone().with_material(material_id),
        }
    }

    /// Set vertex normals (optional, for lighting).
    fn with_normals(&self, normals: Vec<f32>) -> Self {
        Self {
            inner: self.inner.clone().with_normals(normals),
        }
    }

    /// Set per-vertex scalars (for colormap materials).
    fn with_scalars(&self, scalars: Vec<f32>) -> Self {
        Self {
            inner: self.inner.clone().with_scalars(scalars),
        }
    }

    /// Number of triangles.
    #[getter]
    fn triangle_count(&self) -> usize {
        self.inner.indices.len() / 3
    }

    /// Number of vertices.
    #[getter]
    fn vertex_count(&self) -> usize {
        self.inner.positions.len() / 3
    }

    fn __repr__(&self) -> String {
        format!("Mesh(vertices={}, triangles={})",
            self.vertex_count(), self.triangle_count())
    }
}

// =============================================================================
// PointCloud
// =============================================================================

/// Point cloud geometry.
#[pyclass]
#[derive(Clone)]
pub struct PointCloud {
    inner: RustPointCloud,
}

#[pymethods]
impl PointCloud {
    /// Create a point cloud.
    ///
    /// Args:
    ///     positions: Flat list of point positions [x0,y0,z0, x1,y1,z1, ...]
    ///     point_size: Size of points in pixels (default: 5.0)
    #[new]
    #[pyo3(signature = (positions, point_size=5.0))]
    fn new(positions: Vec<f32>, point_size: f32) -> Self {
        Self {
            inner: RustPointCloud::new(positions, point_size),
        }
    }

    /// Set the material ID.
    fn with_material(&self, material_id: &str) -> Self {
        Self {
            inner: self.inner.clone().with_material(material_id),
        }
    }

    /// Set per-point scalars.
    fn with_scalars(&self, scalars: Vec<f32>) -> Self {
        Self {
            inner: self.inner.clone().with_scalars(scalars),
        }
    }

    /// Number of points.
    #[getter]
    fn point_count(&self) -> usize {
        self.inner.positions.len() / 3
    }

    fn __repr__(&self) -> String {
        format!("PointCloud(points={}, size={})",
            self.point_count(), self.inner.point_size)
    }
}

// =============================================================================
// Polyline
// =============================================================================

/// Polyline (connected line segments).
#[pyclass]
#[derive(Clone)]
pub struct Polyline {
    inner: RustPolyline,
}

#[pymethods]
impl Polyline {
    /// Create a polyline.
    ///
    /// Args:
    ///     positions: Flat list of vertex positions [x0,y0,z0, x1,y1,z1, ...]
    ///     line_width: Width of lines in pixels (default: 1.0)
    #[new]
    #[pyo3(signature = (positions, line_width=1.0))]
    fn new(positions: Vec<f32>, line_width: f32) -> Self {
        Self {
            inner: RustPolyline::new(positions, line_width),
        }
    }

    /// Set the material ID.
    fn with_material(&self, material_id: &str) -> Self {
        Self {
            inner: self.inner.clone().with_material(material_id),
        }
    }

    fn __repr__(&self) -> String {
        let vertex_count = self.inner.positions.len() / 3;
        format!("Polyline(vertices={}, width={})",
            vertex_count, self.inner.line_width)
    }
}

// =============================================================================
// Materials
// =============================================================================

/// Solid color material.
#[pyclass]
#[derive(Clone)]
pub struct SolidMaterial {
    inner: RustSolidMaterial,
}

#[pymethods]
impl SolidMaterial {
    /// Create a solid color material.
    ///
    /// Args:
    ///     id: Material identifier
    ///     rgb: Color as [r, g, b] (0-1 range)
    #[new]
    fn new(id: &str, rgb: [f32; 3]) -> Self {
        Self {
            inner: RustSolidMaterial::new(id, rgb),
        }
    }

    /// Create with alpha transparency.
    #[staticmethod]
    fn with_alpha(id: &str, rgba: [f32; 4]) -> Self {
        Self {
            inner: RustSolidMaterial::with_alpha(id, rgba),
        }
    }

    fn __repr__(&self) -> String {
        format!("SolidMaterial(id='{}', color={:?})",
            self.inner.id, self.inner.color)
    }
}

/// Scalar-mapped colormap material.
#[pyclass]
#[derive(Clone)]
pub struct ScalarMappedMaterial {
    inner: RustScalarMappedMaterial,
}

#[pymethods]
impl ScalarMappedMaterial {
    /// Create a scalar-mapped material.
    ///
    /// Args:
    ///     id: Material identifier
    ///     colormap: Name of colormap ("viridis", "plasma", "magma", "inferno", "cividis")
    ///     range: [min, max] for scalar mapping
    #[new]
    fn new(id: &str, colormap: &str, range: [f32; 2]) -> Self {
        Self {
            inner: RustScalarMappedMaterial::new(id, colormap, range),
        }
    }

    fn __repr__(&self) -> String {
        format!("ScalarMappedMaterial(id='{}', colormap='{}', range={:?})",
            self.inner.id, self.inner.colormap, self.inner.range)
    }
}

// =============================================================================
// Volume & Marching Cubes
// =============================================================================

/// 3D scalar volume for isosurface extraction.
#[pyclass]
#[derive(Clone)]
pub struct Volume {
    inner: RustVolume,
}

#[pymethods]
impl Volume {
    /// Create a volume from scalar values.
    ///
    /// Args:
    ///     values: Flat array of scalar values (row-major: x + y*nx + z*nx*ny)
    ///     dimensions: [nx, ny, nz] grid dimensions
    ///     spacing: [dx, dy, dz] grid spacing (default: [1,1,1])
    ///     origin: [ox, oy, oz] world-space origin (default: [0,0,0])
    #[new]
    #[pyo3(signature = (values, dimensions, spacing=None, origin=None))]
    fn new(
        values: Vec<f32>,
        dimensions: [usize; 3],
        spacing: Option<[f32; 3]>,
        origin: Option<[f32; 3]>,
    ) -> PyResult<Self> {
        let expected = dimensions[0] * dimensions[1] * dimensions[2];
        if values.len() != expected {
            return Err(PyValueError::new_err(format!(
                "Expected {} values for dimensions {:?}, got {}",
                expected, dimensions, values.len()
            )));
        }
        Ok(Self {
            inner: RustVolume::new(
                values,
                dimensions,
                spacing.unwrap_or([1.0, 1.0, 1.0]),
                origin.unwrap_or([0.0, 0.0, 0.0]),
            ),
        })
    }

    /// Compute gradient magnitude field.
    fn gradient_magnitude(&self) -> Self {
        Self {
            inner: self.inner.gradient_magnitude(),
        }
    }

    /// Compute Laplacian field.
    fn laplacian(&self) -> Self {
        Self {
            inner: self.inner.laplacian(),
        }
    }

    /// Normalize values to [0, 1].
    fn normalize(&self) -> Self {
        Self {
            inner: self.inner.normalize(),
        }
    }

    /// Get (min, max) value range.
    fn value_range(&self) -> (f32, f32) {
        self.inner.value_range()
    }

    fn __repr__(&self) -> String {
        let (min, max) = self.value_range();
        format!("Volume(dimensions={:?}, range=[{:.3}, {:.3}])",
            self.inner.dimensions, min, max)
    }
}

/// Extract isosurface from volume using Marching Cubes.
///
/// Args:
///     volume: Volume to extract from
///     iso_value: Scalar value for the isosurface
///
/// Returns:
///     Mesh with positions, normals, and indices
#[pyfunction]
fn marching_cubes(volume: &Volume, iso_value: f32) -> Mesh {
    Mesh {
        inner: rust_marching_cubes(&volume.inner, iso_value),
    }
}

/// Extract multiple isosurfaces from volume.
///
/// Args:
///     volume: Volume to extract from
///     iso_values: List of scalar values
///
/// Returns:
///     List of (iso_value, Mesh) tuples
#[pyfunction]
fn marching_cubes_multi(volume: &Volume, iso_values: Vec<f32>) -> Vec<(f32, Mesh)> {
    rust_marching_cubes_multi(&volume.inner, &iso_values)
        .into_iter()
        .map(|s| (s.iso_value, Mesh { inner: s.mesh }))
        .collect()
}

// =============================================================================
// Scene
// =============================================================================

/// Scene container for geometry, materials, and camera.
#[pyclass]
pub struct Scene {
    camera: RustCamera,
    bounds_min: [f32; 3],
    bounds_max: [f32; 3],
    meshes: Vec<RustMesh>,
    point_clouds: Vec<RustPointCloud>,
    polylines: Vec<RustPolyline>,
    materials: Vec<RustMaterial>,
    light: Option<RustLight>,
}

#[pymethods]
impl Scene {
    /// Create a new scene.
    ///
    /// Args:
    ///     camera: Camera for viewing
    ///     bounds: Scene bounds as ([min_x, min_y, min_z], [max_x, max_y, max_z])
    #[new]
    fn new(camera: &Camera, bounds: ([f32; 3], [f32; 3])) -> Self {
        Self {
            camera: camera.inner.clone(),
            bounds_min: bounds.0,
            bounds_max: bounds.1,
            meshes: Vec::new(),
            point_clouds: Vec::new(),
            polylines: Vec::new(),
            materials: Vec::new(),
            light: None,
        }
    }

    /// Add a mesh to the scene.
    fn add_mesh(&mut self, mesh: &Mesh) {
        self.meshes.push(mesh.inner.clone());
    }

    /// Add a point cloud to the scene.
    fn add_points(&mut self, points: &PointCloud) {
        self.point_clouds.push(points.inner.clone());
    }

    /// Add a polyline to the scene.
    fn add_polyline(&mut self, polyline: &Polyline) {
        self.polylines.push(polyline.inner.clone());
    }

    /// Add a solid color material.
    fn add_solid_material(&mut self, material: &SolidMaterial) {
        self.materials.push(RustMaterial::Solid(material.inner.clone()));
    }

    /// Add a scalar-mapped material.
    fn add_scalar_material(&mut self, material: &ScalarMappedMaterial) {
        self.materials.push(RustMaterial::ScalarMapped(material.inner.clone()));
    }

    /// Set the scene light.
    fn set_light(&mut self, light: &Light) {
        self.light = Some(light.inner.clone());
    }

    fn __repr__(&self) -> String {
        format!(
            "Scene(meshes={}, points={}, lines={}, materials={})",
            self.meshes.len(),
            self.point_clouds.len(),
            self.polylines.len(),
            self.materials.len()
        )
    }
}

impl Scene {
    fn to_rust_scene(&self) -> RustScene {
        let mut scene = RustScene::new(
            self.camera.clone(),
            RustBounds {
                min: self.bounds_min,
                max: self.bounds_max,
            },
        );

        for material in &self.materials {
            scene.materials.push(material.clone());
        }

        for mesh in &self.meshes {
            scene = scene.add_mesh(mesh.clone());
        }

        for pc in &self.point_clouds {
            scene = scene.add_point_cloud(pc.clone());
        }

        for line in &self.polylines {
            scene = scene.add_polyline(line.clone());
        }

        if let Some(light) = &self.light {
            scene.light = Some(light.clone());
        }

        scene
    }
}

// =============================================================================
// Rendering
// =============================================================================

/// Render configuration.
#[pyclass]
#[derive(Clone)]
pub struct RenderConfig {
    /// Image width in pixels.
    #[pyo3(get, set)]
    pub width: u32,
    /// Image height in pixels.
    #[pyo3(get, set)]
    pub height: u32,
    /// Background color as [r, g, b, a] (0-1 range).
    #[pyo3(get, set)]
    pub background: [f32; 4],
}

#[pymethods]
impl RenderConfig {
    /// Create a render configuration.
    ///
    /// Args:
    ///     width: Image width (default: 512)
    ///     height: Image height (default: 512)
    ///     background: Background color [r,g,b,a] (default: dark gray)
    #[new]
    #[pyo3(signature = (width=512, height=512, background=None))]
    fn new(width: u32, height: u32, background: Option<[f32; 4]>) -> Self {
        Self {
            width,
            height,
            background: background.unwrap_or([0.1, 0.1, 0.15, 1.0]),
        }
    }

    fn __repr__(&self) -> String {
        format!("RenderConfig({}x{}, bg={:?})",
            self.width, self.height, self.background)
    }
}

/// Render scene to PNG bytes.
///
/// Args:
///     scene: Scene to render
///     config: Render configuration (optional)
///
/// Returns:
///     PNG image data as bytes
#[pyfunction]
#[pyo3(signature = (scene, config=None))]
fn render_to_png(scene: &Scene, config: Option<&RenderConfig>) -> PyResult<Vec<u8>> {
    let rust_config = config
        .map(|c| RustRenderConfig {
            width: c.width,
            height: c.height,
            background: c.background,
        })
        .unwrap_or(RustRenderConfig {
            width: 512,
            height: 512,
            background: [0.1, 0.1, 0.15, 1.0],
        });

    let rust_scene = scene.to_rust_scene();

    rust_render_to_png(&rust_scene, &rust_config)
        .map_err(|e| PyValueError::new_err(format!("Render error: {:?}", e)))
}

/// Save PNG bytes to a file.
///
/// Args:
///     data: PNG bytes from render_to_png
///     path: Output file path
#[pyfunction]
fn save_png(data: Vec<u8>, path: &str) -> PyResult<()> {
    std::fs::write(path, data)
        .map_err(|e| PyValueError::new_err(format!("Failed to write file: {}", e)))
}

// =============================================================================
// Module Definition
// =============================================================================

/// Frustum: GPU-accelerated scientific 3D visualization.
///
/// Example:
///     import frustum
///
///     # Create scene
///     cam = frustum.Camera.perspective([5, 3, 5], [0, 0, 0])
///     scene = frustum.Scene(cam, bounds=([-1,-1,-1], [1,1,1]))
///
///     # Add geometry
///     mesh = frustum.Mesh([0,0,0, 1,0,0, 0.5,1,0], [0,1,2])
///     scene.add_mesh(mesh)
///
///     # Render
///     png = frustum.render_to_png(scene)
///     frustum.save_png(png, "output.png")
#[pymodule]
fn frustum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Classes
    m.add_class::<Camera>()?;
    m.add_class::<Light>()?;
    m.add_class::<Mesh>()?;
    m.add_class::<PointCloud>()?;
    m.add_class::<Polyline>()?;
    m.add_class::<SolidMaterial>()?;
    m.add_class::<ScalarMappedMaterial>()?;
    m.add_class::<Volume>()?;
    m.add_class::<Scene>()?;
    m.add_class::<RenderConfig>()?;

    // Functions
    m.add_function(wrap_pyfunction!(marching_cubes, m)?)?;
    m.add_function(wrap_pyfunction!(marching_cubes_multi, m)?)?;
    m.add_function(wrap_pyfunction!(render_to_png, m)?)?;
    m.add_function(wrap_pyfunction!(save_png, m)?)?;

    Ok(())
}
