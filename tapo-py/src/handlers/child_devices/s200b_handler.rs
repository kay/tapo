use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};
use tapo::responses::{S200BLog, S200BResult, TriggerLogsResult};
use tapo::S200BHandler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "S200BHandler")]
pub struct PyS200BHandler {
    handler: Arc<S200BHandler>,
}

impl PyS200BHandler {
    pub fn new(handler: S200BHandler) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyS200BHandler {
    pub async fn get_device_info(&self) -> PyResult<S200BResult> {
        let handler = self.handler.clone();
        call_handler_method!(handler.deref(), S200BHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(handler.deref(), S200BHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsS200BResult> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.deref(),
            S200BHandler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyo3::prelude::pyclass(get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsS200BResult {
    start_id: u64,
    sum: u64,
    logs: Vec<S200BLog>,
}

impl From<TriggerLogsResult<S200BLog>> for TriggerLogsS200BResult {
    fn from(result: TriggerLogsResult<S200BLog>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

#[pyo3::pymethods]
impl TriggerLogsS200BResult {
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        tapo::python::serde_object_to_py_dict(py, &value)
    }
}