use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use std::collections::HashMap;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::testrun::{Outcome, Testrun};

// from https://gist.github.com/scott-codecov/311c174ecc7de87f7d7c50371c6ef927#file-cobertura-rs-L18-L31
fn attributes_map(attributes: Attributes) -> HashMap<String, String> {
    return attributes
        .filter_map(|a| {
            if let Ok(attr) = a {
                let bytes = attr.value.into_owned();
                let value = String::from_utf8(bytes).unwrap();
                let key = String::from_utf8(attr.key.local_name().as_ref().to_vec()).unwrap();
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
}

fn populate(attr_hm: &HashMap<String, String>, testsuite: String) -> Testrun {
    let name = format!(
        "{}::{}",
        attr_hm.get("classname").unwrap(),
        attr_hm.get("name").unwrap()
    );

    let duration = attr_hm.get("time").unwrap().to_string().parse().unwrap();

    Testrun {
        name,
        duration,
        outcome: Outcome::Pass,
        testsuite: testsuite,
    }
}

#[pyfunction]
pub fn parse_junit_xml(file_bytes: Vec<u8>) -> PyResult<Vec<Testrun>> {
    let file_string = String::from_utf8_lossy(&file_bytes).into_owned();
    let mut reader = Reader::from_str(file_string.as_str());
    reader.trim_text(true);

    let mut list_of_test_runs = Vec::new();

    let mut buf = Vec::new();

    let mut saved_testrun: Option<Testrun> = None;

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
                    saved_testrun = Some(populate(&attr_hm, curr_testsuite.clone()));
                }
                b"skipped" => {
                    let mut testrun = saved_testrun.unwrap();
                    testrun.outcome = Outcome::Skip;
                    saved_testrun = Some(testrun);
                }
                b"error" => {
                    let mut testrun = saved_testrun.unwrap();
                    testrun.outcome = Outcome::Error;
                    saved_testrun = Some(testrun);
                }
                b"failure" => {
                    let mut testrun = saved_testrun.unwrap();
                    testrun.outcome = Outcome::Failure;
                    saved_testrun = Some(testrun);
                }
                b"testsuite" => {
                    let attr_hm = attributes_map(e.attributes());

                    curr_testsuite = attr_hm.get("name").unwrap().to_string();
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"testcase" => {
                    list_of_test_runs.push(saved_testrun.unwrap());
                    saved_testrun = None;
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"testcase" => {
                    let attr_hm = attributes_map(e.attributes());
                    list_of_test_runs.push(populate(&attr_hm, curr_testsuite.clone()));
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
