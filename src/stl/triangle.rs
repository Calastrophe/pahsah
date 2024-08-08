use glam::Vec3;
use pyo3::prelude::*;

/// A type which represents each individual triangle in a STL file.
///
/// In certain STL software, the attribute bytes may be used for storing a color.
#[pyclass]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// The normal vector of the given triangle.
    pub normal: Vec3,
    /// The first vertex of the given triangle.
    pub v1: Vec3,
    /// The second vertex of the given triangle.
    pub v2: Vec3,
    /// The third vertex of the given triangle.
    pub v3: Vec3,
    /// Attribute bytes, depending on the software, these have different purposes.
    pub attribute: u16,
}

impl Triangle {
    /// Create a new triangle from the given components.
    pub const fn new(normal: Vec3, v1: Vec3, v2: Vec3, v3: Vec3, attribute: u16) -> Self {
        Triangle {
            normal,
            v1,
            v2,
            v3,
            attribute,
        }
    }
}

type Vector3 = [f32; 3];

#[pymethods]
impl Triangle {
    #[new]
    fn py_new(normal: Vector3, v1: Vector3, v2: Vector3, v3: Vector3, attribute: u16) -> Self {
        Triangle {
            normal: normal.into(),
            v1: v1.into(),
            v2: v2.into(),
            v3: v3.into(),
            attribute,
        }
    }

    // pyo3 doesn't allow macros inside of their own macro??!!
    // instead we just repeat code, cringe.

    #[getter]
    fn get_normal(&self) -> PyResult<Vector3> {
        Ok(self.normal.to_array())
    }

    #[setter]
    fn set_normal(&mut self, value: Vector3) -> PyResult<()> {
        self.normal = value.into();
        Ok(())
    }

    #[getter]
    fn get_v1(&self) -> PyResult<Vector3> {
        Ok(self.v1.to_array())
    }

    #[setter]
    fn set_v1(&mut self, value: Vector3) -> PyResult<()> {
        self.v1 = value.into();
        Ok(())
    }

    #[getter]
    fn get_v2(&self) -> PyResult<Vector3> {
        Ok(self.v1.to_array())
    }

    #[setter]
    fn set_v2(&mut self, value: Vector3) -> PyResult<()> {
        self.v1 = value.into();
        Ok(())
    }

    #[getter]
    fn get_v3(&self) -> PyResult<Vector3> {
        Ok(self.v1.to_array())
    }

    #[setter]
    fn set_v3(&mut self, value: Vector3) -> PyResult<()> {
        self.v1 = value.into();
        Ok(())
    }

    #[getter]
    fn get_attribute(&self) -> PyResult<u16> {
        Ok(self.attribute)
    }

    #[setter]
    fn set_attribute(&mut self, value: u16) -> PyResult<()> {
        self.attribute = value;
        Ok(())
    }
}
