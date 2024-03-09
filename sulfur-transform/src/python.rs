use std::io::Read;
use pyo3::{Py, PyAny, Python};
use pyo3::prelude::PyModule;
use pyo3::types::PyTuple;
use serde_json::Value;
use sulfur_common::config::config::Transform;

pub fn transform(data: Value, transform: Option<Transform>) -> Value {
    if transform.is_some() {
        let t = transform.unwrap();
        let mut t_content = String::new();
        let mut f = std::fs::File::open(t.file.unwrap()).expect("Could not open file.");
        f.read_to_string(&mut t_content).expect("TODO: panic message");

        return Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                t_content.as_str(),
                "",
                "",
            ).unwrap()
                .getattr(t.r#fn.unwrap().as_str()).unwrap()
                .into();

            let args = PyTuple::new(py, &[data.clone().to_string()]);
            let py_object = fun.call1(py, args).expect("TODO: panic message");

            let transformed_data = serde_json::from_str::<Value>(py_object.to_string().as_str()).unwrap();

            return transformed_data;
        });
    }

    data
}