use glam::Vec3;
use std::fmt::Display;

/// A type which represents each individual triangle in a STL file.
///
/// In certain STL software, the attribute bytes may be used for storing a color.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// The normal vector of the given triangle.
    pub normal_vector: Vec3,
    /// The first vertex of the given triangle.
    pub v1: Vec3,
    /// The second vertex of the given triangle.
    pub v2: Vec3,
    /// The third vertex of the given triangle.
    pub v3: Vec3,
    /// Attribute bytes, depending on the software, these have different purposes.
    pub attribute: u16,
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.normal_vector, self.v1, self.v2, self.v3, self.attribute
        )
    }
}

impl Triangle {
    /// Create a new triangle from the given components.
    pub const fn new(normal_vector: Vec3, v1: Vec3, v2: Vec3, v3: Vec3, attribute: u16) -> Self {
        Triangle {
            normal_vector,
            v1,
            v2,
            v3,
            attribute,
        }
    }
}
