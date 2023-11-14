use pyo3::class::basic::CompareOp;
use pyo3::exceptions::PyException;
use pyo3::{prelude::*, pyclass};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;

use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

fn s(string_slice: &str) -> String {
    string_slice.to_string()
}

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

#[derive(Clone, Debug)]
#[pyclass]
struct Testrun {
    name: String,
    duration: String,
    outcome: String,
    testsuite: String,
}

impl Testrun {
    fn empty() -> Testrun {
        return Testrun {
            name: s(""),
            duration: s(""),
            outcome: s(""),
            testsuite: s(""),
        };
    }

    fn populate(&mut self, attr_hm: &HashMap<String, String>, curr_testsuite: String) {
        let name = format!(
            "{}::{}",
            attr_hm.get("classname").unwrap().to_string(),
            attr_hm.get("name").unwrap().to_string()
        );
        self.name = name;

        let duration = attr_hm.get("time").unwrap().to_string();
        self.duration = duration;

        self.outcome = s("pass");

        self.testsuite = curr_testsuite
    }
}

#[pymethods]
impl Testrun {
    #[new]
    fn new(name: String, duration: String, outcome: String, testsuite: String) -> Self {
        Self {
            name,
            duration,
            outcome,
            testsuite,
        }
    }

    fn __repr__(&self) -> String {
        return format!(
            "({}, {}, {}, {})",
            self.name, self.outcome, self.duration, self.testsuite
        );
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.name.hash(&mut hasher);
        self.duration.hash(&mut hasher);
        self.outcome.hash(&mut hasher);
        self.testsuite.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.name == other.name
                && self.outcome == other.outcome
                && self.duration == other.duration
                && self.testsuite == other.testsuite),
            _ => todo!(),
        }
    }
}

#[pyfunction]
fn parse_junit_xml(xml_string: String) -> PyResult<Vec<Testrun>> {
    let mut reader = Reader::from_file(&xml_string).unwrap();
    reader.trim_text(true);

    let mut list_of_test_runs = Vec::new();

    let mut buf = Vec::new();

    let mut saved_testrun = Testrun::empty();

    let mut curr_testsuite = String::new();

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

                    saved_testrun.populate(&attr_hm, curr_testsuite.clone());
                }
                b"skipped" => {
                    saved_testrun.outcome = s("skipped");
                }
                b"error" => {
                    saved_testrun.outcome = s("error");
                }
                b"failure" => {
                    saved_testrun.outcome = s("failure");
                }
                b"testsuite" => {
                    let attr_hm = attributes_map(e.attributes());

                    curr_testsuite = attr_hm.get("name").unwrap().to_string();
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"testcase" => {
                    list_of_test_runs.push(saved_testrun.clone());
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let attr_hm = attributes_map(e.attributes());

                    saved_testrun.populate(&attr_hm, curr_testsuite.clone());

                    list_of_test_runs.push(saved_testrun.clone());
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
    m.add_class::<Testrun>()?;
    m.add_function(wrap_pyfunction!(parse_junit_xml, m)?)?;
    Ok(())
}
