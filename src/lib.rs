//! TODO: The STL parser portion of the library
#![forbid(rust_2018_idioms, unsafe_code)]
#![deny(missing_debug_implementations, unused_results)]

use pyo3::{prelude::*, wrap_pymodule};
use stl::{Triangle, STL};

pub mod stl;

#[pymodule]
fn pahsah(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<STL>()?;
    m.add_class::<Triangle>()?;
    m.add_wrapped(wrap_pymodule!(stl::ascii::ascii))?;
    m.add_wrapped(wrap_pymodule!(stl::parallel::parallel))?;
    m.add_function(wrap_pyfunction!(stl::parse_file, m)?)?;
    m.add_function(wrap_pyfunction!(stl::parse_bytes, m)?)
}
