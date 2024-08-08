use super::{STLReaderExt, STL, TRIANGLE_SIZE};
use byteorder::{LittleEndian, ReadBytesExt};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom},
};

#[pymodule]
pub fn parallel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)
}

#[pyfunction]
pub fn parse(path: &str) -> io::Result<STL> {
    let mut file = File::open(path)?;

    let mut contents = Vec::new();

    let _ = file.read_to_end(&mut contents)?;

    let (header, contents) = contents.split_at(84);

    let mut header = Cursor::new(header);

    let _ = header.seek(SeekFrom::Current(80))?;

    let num_triangles = header.read_u32::<LittleEndian>()? as usize;

    let chunk_size = num_triangles / 4;

    let triangles = contents
        .par_chunks(chunk_size * TRIANGLE_SIZE)
        .try_fold(
            || Vec::with_capacity(chunk_size),
            |mut acc, chunk| {
                let mut reader = &chunk[..];

                while reader.len() >= TRIANGLE_SIZE {
                    acc.push(reader.read_triangle()?);
                }

                io::Result::Ok(acc)
            },
        )
        .try_reduce(
            || Vec::with_capacity(num_triangles),
            |mut acc, triangles| {
                acc.extend(triangles);
                Ok(acc)
            },
        )?;

    Ok(STL { triangles })
}
