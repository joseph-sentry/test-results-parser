use pyo3::prelude::*;

mod helpers;
mod junit;
mod testrun;

/// A Python module implemented in Rust.
#[pymodule]
fn testing_result_parsers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<testrun::Testrun>()?;
    m.add_function(wrap_pyfunction!(junit::parse_junit_xml, m)?)?;
    Ok(())
}
