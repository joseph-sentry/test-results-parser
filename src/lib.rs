use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::path::Path;

// from https://gist.github.com/scott-codecov/311c174ecc7de87f7d7c50371c6ef927#file-cobertura-rs-L18-L31
fn attributes_map(attributes: Attributes) -> HashMap<String, String> {
    return attributes
        .filter_map(|a| {
            if let Ok(attr) = a {
                let bytes = attr.value.into_owned();
                let value = String::from_utf8(bytes).unwrap();
                let key = String::from_utf8(attr.key.local_name().as_ref().to_vec()).unwrap();
                return Some((key, value));
            } else {
                return None;
            }
        })
        .collect::<HashMap<_, _>>();
}

#[pyfunction]
fn parse_junit_xml(path_string: String) -> PyResult<Vec<HashMap<&'static str, String>>> {
    let mut reader = match Reader::from_file(Path::new(&path_string)) {
        Ok(reader) => Ok(reader),
        Err(_) => Err(PyException::new_err(format!(
            "Error creating reader from file {}",
            &path_string
        ))),
    }?;
    reader.trim_text(true);

    let mut list_of_test_runs = Vec::new();

    let mut buf = Vec::new();

    let mut saved_hm: HashMap<&str, String> = HashMap::new();
    let mut saved_outcome = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                break Err(PyException::new_err(format!(
                    "Error at position: {} {:?}",
                    reader.buffer_position(),
                    e
                )))
            }
            Ok(Event::Eof) => {
                break Ok(list_of_test_runs);
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let attr_hm = attributes_map(e.attributes());

                    let name = format!(
                        "{}.{}",
                        attr_hm.get("classname").unwrap().to_string(),
                        attr_hm.get("name").unwrap().to_string()
                    );
                    saved_hm.insert("name", name);

                    let duration = attr_hm.get("time").unwrap().to_string();
                    saved_hm.insert("duration", duration);

                    saved_outcome = String::from("pass");
                }
                b"skipped" => {
                    saved_outcome = String::from("skipped");
                }
                b"error" => {
                    saved_outcome = String::from("error");
                }
                b"failure" => {
                    saved_outcome = String::from("failure");
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"testcase" => {
                    saved_hm.insert("outcome", saved_outcome.clone());

                    list_of_test_runs.push(saved_hm.clone());
                    saved_hm.clear();
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let attr_hm = attributes_map(e.attributes());

                    let name = format!(
                        "{}.{}",
                        attr_hm.get("classname").unwrap().to_string(),
                        attr_hm.get("name").unwrap().to_string()
                    );
                    saved_hm.insert("name", name);

                    let duration = attr_hm.get("time").unwrap().to_string();
                    saved_hm.insert("duration", duration);

                    saved_hm.insert("outcome", String::from("pass"));

                    list_of_test_runs.push(saved_hm.clone());
                    saved_hm.clear();
                }
                _ => (),
            },
            Ok(Event::Text(_)) => (),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        buf.clear()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn testing_result_parsers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_junit_xml, m)?)?;
    Ok(())
}
