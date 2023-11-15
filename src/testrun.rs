use pyo3::class::basic::CompareOp;
use pyo3::{prelude::*, pyclass};

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

use crate::helpers::s;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

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
        return Testrun {
            name: s(""),
            duration: s(""),
            outcome: s(""),
            testsuite: s(""),
        };
    }

    pub fn populate(&mut self, attr_hm: &HashMap<String, String>, curr_testsuite: String) {
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
