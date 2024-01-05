use pyo3::prelude::*;

mod helpers;
mod junit;
mod pytest_reportlog;
mod testrun;
mod vitest_json;

/// A Python module implemented in Rust.
#[pymodule]
fn test_results_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("ParserError", _py.get_type::<helpers::ParserError>())?;
    m.add_class::<testrun::Testrun>()?;
    m.add_class::<testrun::Outcome>()?;

    m.add_function(wrap_pyfunction!(junit::parse_junit_xml, m)?)?;
    m.add_function(wrap_pyfunction!(
        pytest_reportlog::parse_pytest_reportlog,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(vitest_json::parse_vitest_json, m)?)?;

    Ok(())
}
