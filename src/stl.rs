use binary::STLReaderExt;
use byteorder::{LittleEndian, ReadBytesExt};
use glam::Vec3;
use pyo3::prelude::*;
use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek, SeekFrom},
};

pub const TRIANGLE_SIZE: usize = 50;

pub mod ascii;
pub mod binary;
pub mod parallel;
pub mod triangle;

pub use triangle::Triangle;

/// A struct holding all of the triangles which make up the STL file.
#[pyclass]
#[derive(Debug, Default)]
pub struct STL {
    pub triangles: Vec<Triangle>,
}

#[pymethods]
impl STL {
    #[getter]
    fn get_triangles(&self) -> PyResult<Vec<Triangle>> {
        Ok(self.triangles.clone())
    }

    #[setter]
    fn set_triangles(&mut self, triangles: Vec<Triangle>) -> PyResult<()> {
        self.triangles = triangles;
        Ok(())
    }

    fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

/// Underlying implementation for reading an STL file from a given source which implements Read
/// and Seek.
fn parse_underlying<R: Read + Seek>(reader: R) -> io::Result<STL> {
    let mut reader = BufReader::with_capacity(TRIANGLE_SIZE, reader);

    // Skip over the STL header
    let _ = reader.seek(SeekFrom::Current(80))?;

    let size = reader.read_u32::<LittleEndian>()? as usize;

    let triangles = (0..size).try_fold(Vec::with_capacity(size), |mut triangles, _| {
        triangles.push(reader.read_triangle()?);
        io::Result::Ok(triangles)
    })?;

    Ok(STL { triangles })
}

#[pyfunction]
/// Parses a given path to a binary STL file.
pub fn parse_file(path: &str) -> io::Result<STL> {
    parse_underlying(File::open(path)?)
}

#[pyfunction]
/// Parses a given array of bytes which make up a binary STL file.
pub fn parse_bytes(bytes: &[u8]) -> io::Result<STL> {
    parse_underlying(Cursor::new(bytes))
}
