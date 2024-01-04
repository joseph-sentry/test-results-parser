use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

use serde::{Deserialize, Serialize};

use crate::testrun::{Outcome, Testrun};

#[derive(Serialize, Deserialize, Debug)]
struct AssertionResult {
    #[serde(rename = "ancestorTitles")]
    ancestor_titles: Vec<String>,
    #[serde(rename = "fullName")]
    full_name: String,
    status: String,
    title: String,
    #[serde(rename = "duration")]
    duration_milliseconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct VitestResult {
    #[serde(rename = "assertionResults")]
    assertion_results: Vec<AssertionResult>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VitestReport {
    #[serde(rename = "testResults")]
    test_results: Vec<VitestResult>,
}

#[pyfunction]
pub fn parse_vitest_json(file_bytes: Vec<u8>) -> PyResult<Vec<Testrun>> {
    let file_string = String::from_utf8_lossy(&file_bytes).into_owned();

    let val: VitestReport = serde_json::from_str(file_string.as_str())
        .map_err(|err| PyErr::new::<PyValueError, _>(err.to_string()))?;

    let testruns: Result<Vec<Testrun>, _> = val
        .test_results
        .into_iter()
        .flat_map(|result| {
            result
                .assertion_results
                .into_iter()
                .map(move |aresult| {
                    Ok(Testrun {
                        name: aresult.full_name,
                        duration: aresult.duration_milliseconds as f64 / 1000.0,
                        outcome: (match aresult.status.as_str() {
                            "failed" => Outcome::Failure,
                            "pending" => Outcome::Skip,
                            "passed" => Outcome::Pass,
                            _ => return Err(PyRuntimeError::new_err("oh noooooooo")),
                        }),
                        testsuite: result.name.clone(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(testruns?)
}
