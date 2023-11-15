use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use std::collections::HashMap;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::helpers::s;
use crate::testrun::Testrun;

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

fn populate(testrun: &mut Testrun, attr_hm: &HashMap<String, String>, curr_testsuite: String) {
    let name = format!(
        "{}::{}",
        attr_hm.get("classname").unwrap().to_string(),
        attr_hm.get("name").unwrap().to_string()
    );
    testrun.name = name;

    let duration = attr_hm.get("time").unwrap().to_string();
    testrun.duration = duration;

    testrun.outcome = s("pass");

    testrun.testsuite = curr_testsuite
}

#[pyfunction]
pub fn parse_junit_xml(filename: String) -> PyResult<Vec<Testrun>> {
    let mut reader = Reader::from_file(&filename).unwrap();
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

                    populate(&mut saved_testrun, &attr_hm, curr_testsuite.clone());
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

                    populate(&mut saved_testrun, &attr_hm, curr_testsuite.clone());

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
