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
pub fn parse_pytest_reportlog(file_bytes: Vec<u8>) -> PyResult<Vec<Testrun>> {
    let mut testruns: Vec<Testrun> = Vec::new();

    let file_string = String::from_utf8_lossy(&file_bytes).into_owned();

    let mut saved_start_time: Option<f64> = Some(0.0);

    let mut lineno = 0;

    let string_lines = file_string.lines();

    for line in string_lines {
        let val: PytestLine = serde_json::from_str(line)
            .map_err(|err| PyRuntimeError::new_err(format!("Error parsing json line  {}", err)))?;

        if val.report_type == "TestReport" {
            match val.when.as_str() {
                "setup" => {
                    saved_start_time = Some(val.start);
                }
                "teardown" => {
                    let location = val.location.ok_or(PyRuntimeError::new_err(format!(
                        "Error reading location on line number {}",
                        lineno
                    )))?;
                    let name = location.2;
                    let testsuite = location.0;
                    let outcome = match val.outcome.as_str() {
                        "passed" => Outcome::Pass,
                        "failed" => Outcome::Failure,
                        "skipped" => Outcome::Skip,
                        x => {
                            return Err(PyRuntimeError::new_err(format!(
                                "Error reading outcome on line number {}. {} is an invalid value",
                                lineno, x
                            )))
                        }
                    };
                    let end_time = val.stop;
                    let start_time = saved_start_time.ok_or(PyRuntimeError::new_err(format!(
                        "Error reading saved start time on line number {}",
                        lineno
                    )))?;

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
    }

    Ok(testruns)
}
