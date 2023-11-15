use pyo3::class::basic::CompareOp;
use pyo3::{prelude::*, pyclass};

use crate::helpers::s;

#[derive(Clone, Debug)]
#[pyclass]
pub struct Testrun {
    pub name: String,
    pub duration: String,
    pub outcome: String,
    pub testsuite: String,
}

impl Testrun {
    pub fn empty() -> Testrun {
        Testrun {
            name: s(""),
            duration: s(""),
            outcome: s(""),
            testsuite: s(""),
        }
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
        format!(
            "({}, {}, {}, {})",
            self.name, self.outcome, self.duration, self.testsuite
        )
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
