use pyo3::prelude::*;

mod helpers;
mod junit;
mod pytest_reportlog;
mod testrun;

/// A Python module implemented in Rust.
#[pymodule]
fn testing_result_parsers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<testrun::Testrun>()?;
    m.add_function(wrap_pyfunction!(junit::parse_junit_xml, m)?)?;
    m.add_function(wrap_pyfunction!(
        pytest_reportlog::parse_pytest_reportlog,
        m
    )?)?;
    Ok(())
}
