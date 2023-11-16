use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::testrun::{Outcome, Testrun};

use serde::{Deserialize, Serialize};

use crate::helpers::s;

#[derive(Serialize, Deserialize)]
struct Location(String, i32, String);

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct PytestLine {
    #[serde(rename = "$report_type")]
    report_type: String,
    start: f64,
    stop: f64,
    location: Option<Location>,
    when: String,
    outcome: String,
}

#[pyfunction]
pub fn parse_pytest_reportlog(file_bytes: Vec<u8>) -> PyResult<Vec<Testrun>> {
    let mut testruns: Vec<Testrun> = Vec::new();

    let file_string = String::from_utf8_lossy(&file_bytes).into_owned();

    let mut saved_start_time: Option<f64> = Some(0.0);

    let mut lineno = 0;

    let string_lines = file_string.lines();

    string_lines.for_each(|line| {
        let val: PytestLine = serde_json::from_str(line)
            .map_err(|err| PyRuntimeError::new_err(format!("Error parsing json line  {}", err)))
            .unwrap();
        if val.report_type == "TestReport" {
            match val.when.as_str() {
                "setup" => {
                    saved_start_time = Some(val.start);
                }
                "teardown" => {
                    let location = val.location.unwrap();
                    let name = location.2;
                    let testsuite = location.0;
                    let outcome = match val.outcome.as_str() {
                        "passed" => Ok(Outcome::Pass),
                        "failed" => Ok(Outcome::Failure),
                        "skipped" => Ok(Outcome::Skip),
                        x => Err(PyRuntimeError::new_err(format!(
                            "Error reading outcome on line number {}. {} is an invalid value",
                            lineno, x
                        ))),
                    }
                    .unwrap();
                    let end_time = val.stop;
                    let start_time = saved_start_time.unwrap();

                    let duration = end_time - start_time;

                    testruns.push(Testrun {
                        name,
                        testsuite,
                        duration,
                        outcome,
                    });
                    saved_start_time = None
                }
                _ => (),
            }
        }
        lineno += 1;
    });

    Ok(testruns)
}
