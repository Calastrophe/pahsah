use super::{Triangle, Vec3};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;

/// A trait which is implemented onto all of io::Read allowing for easy reading of the provided
/// Vec3 and Triangle underlying type provided in the STL structure.
pub trait STLReaderExt {
    /// Reads a Vec3 from the provided type.
    fn read_vector(&mut self) -> io::Result<Vec3>;

    /// Reads a Triangle from the provided type.
    fn read_triangle(&mut self) -> io::Result<Triangle>;
}

impl<T: io::Read> STLReaderExt for T {
    fn read_vector(&mut self) -> io::Result<Vec3> {
        Ok(Vec3::new(
            self.read_f32::<LittleEndian>()?,
            self.read_f32::<LittleEndian>()?,
            self.read_f32::<LittleEndian>()?,
        ))
    }

    fn read_triangle(&mut self) -> io::Result<Triangle> {
        Ok(Triangle::new(
            self.read_vector()?,
            self.read_vector()?,
            self.read_vector()?,
            self.read_vector()?,
            self.read_u16::<LittleEndian>()?,
        ))
    }
}
