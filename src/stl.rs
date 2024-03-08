use binary::STLReaderExt;
use byteorder::{LittleEndian, ReadBytesExt};
use glam::Vec3;
use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

pub mod ascii;
pub mod binary;
pub mod parallel;
pub mod triangle;

pub use triangle::Triangle;

/// A struct holding all of the triangles which make up the STL file.
#[derive(Debug, Default)]
pub struct STL {
    pub triangles: Vec<Triangle>,
}

/// Underlying implementation for reading an STL file from a given source which implements Read
/// and Seek.
fn parse_underlying<R: Read + Seek>(reader: R) -> io::Result<STL> {
    let mut reader = BufReader::with_capacity(std::mem::size_of::<Triangle>(), reader);
    let _ = reader.seek(SeekFrom::Current(80))?;

    let size = reader.read_u32::<LittleEndian>()? as usize;

    let triangles = (0..size).try_fold(Vec::with_capacity(size), |mut triangles, _| {
        triangles.push(reader.read_triangle()?);
        io::Result::Ok(triangles)
    })?;

    Ok(STL { triangles })
}

/// Parses a given path to a file.
pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<STL> {
    parse_underlying(File::open(path)?)
}

/// Parses a given type which provides AsRef<[u8]>.
pub fn parse_bytes<B: AsRef<[u8]>>(bytes: B) -> io::Result<STL> {
    parse_underlying(Cursor::new(bytes))
}
