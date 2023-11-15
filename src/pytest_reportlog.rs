use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::testrun::{Outcome, Testrun};

use serde::{Deserialize, Serialize};

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
pub fn parse_pytest_reportlog(filename: String) -> PyResult<Vec<Testrun>> {
    let mut buf = String::new();
    let f = File::open(&filename)?;

    let mut testruns: Vec<Testrun> = Vec::new();

    let mut reader = BufReader::new(f);

    let mut val: PytestLine;
    let mut saved_start_time: Option<f64> = Some(0.0);

    let mut lineno = 0;

    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                val = serde_json::from_str(buf.trim()).map_err(|err| {
                    PyRuntimeError::new_err(format!("Error parsing json line  {}", err))
                })?;
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
                                    "Error reading outcome on line number {} from {}. {} is an invalid value",
                                    lineno, &filename, x
                                ))),
                            }?;
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
                buf.clear();
                Ok(())
            }
            Err(err) => Err(PyRuntimeError::new_err(format!(
                "Error reading line number {} from {}:  {}",
                lineno, &filename, err
            ))),
        }?;
        lineno += 1;
    }

    Ok(testruns)
}
