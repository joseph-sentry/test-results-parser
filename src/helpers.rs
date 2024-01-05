use pyo3::exceptions::PyException;

pub fn s(string_slice: &str) -> String {
    string_slice.to_string()
}

pyo3::create_exception!(mymodule, ParserError, PyException);
