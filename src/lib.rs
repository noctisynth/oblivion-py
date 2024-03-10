use oblivion::{api, exceptions::PyOblivionException, models::client::Response};
use pyo3::{prelude::*, types::PyDict};
use serde_json::Value;

async fn rget(olps: String, tfo: bool) -> PyResult<Response> {
    match api::get(&olps, tfo).await {
        Ok(res) => Ok(res),
        Err(err) => Err(PyErr::new::<PyOblivionException, _>(format!(
            "Invalid Oblivion: {:?}",
            err
        ))),
    }
}

#[pyfunction]
fn async_get(py: Python, olps: String, tfo: Option<bool>) -> PyResult<&PyAny> {
    let tfo = if tfo.is_none() { false } else { true };
    pyo3_asyncio::tokio::future_into_py(py, rget(olps, tfo))
}

#[pyfunction]
fn get(olps: String, tfo: Option<bool>) -> PyResult<Response> {
    let tfo = if tfo.is_none() { false } else { true };
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { api::get(&olps, tfo).await.unwrap() });
    Ok(Response::new(
        res.header,
        res.content,
        res.olps,
        res.status_code,
    ))
}

async fn rpost(olps: String, data: Value, tfo: bool) -> PyResult<Response> {
    match api::post(&olps, data, tfo).await {
        Ok(res) => Ok(res),
        Err(err) => Err(PyErr::new::<PyOblivionException, _>(format!(
            "Invalid Oblivion: {:?}",
            err
        ))),
    }
}

#[pyfunction]
fn async_post<'a>(
    py: Python<'a>,
    olps: String,
    data: &'a PyDict,
    tfo: Option<bool>,
) -> PyResult<&'a PyAny> {
    let data = Value::from(data.to_string());
    let tfo = if tfo.is_none() { false } else { true };
    pyo3_asyncio::tokio::future_into_py(py, rpost(olps, data, tfo))
}

#[pyfunction]
fn post(olps: String, data: &PyDict, tfo: Option<bool>) -> PyResult<Response> {
    let data = Value::from(data.to_string());
    let tfo = if tfo.is_none() { false } else { true };
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { api::post(&olps, data, tfo).await.unwrap() });
    Ok(Response::new(
        res.header,
        res.content,
        res.olps,
        res.status_code,
    ))
}

async fn rput(olps: String, data: Value, file: Vec<u8>, tfo: bool) -> PyResult<Response> {
    match api::put(&olps, Some(data), file, tfo).await {
        Ok(res) => Ok(res),
        Err(err) => Err(PyErr::new::<PyOblivionException, _>(format!(
            "Invalid Oblivion: {:?}",
            err
        ))),
    }
}

#[pyfunction]
fn async_put<'a>(
    py: Python<'a>,
    olps: String,
    data: &'a PyDict,
    file: Vec<u8>,
    tfo: Option<bool>,
) -> PyResult<&'a PyAny> {
    let data = Value::from(data.to_string());
    let tfo = if tfo.is_none() { false } else { true };
    pyo3_asyncio::tokio::future_into_py(py, rput(olps, data, file, tfo))
}

#[pyfunction]
fn put(olps: String, data: &PyDict, file: Vec<u8>, tfo: Option<bool>) -> PyResult<Response> {
    let data = Value::from(data.to_string());
    let tfo = if tfo.is_none() { false } else { true };
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { api::put(&olps, Some(data), file, tfo).await.unwrap() });
    Ok(Response::new(
        res.header,
        res.content,
        res.olps,
        res.status_code,
    ))
}

#[pymodule]
fn oblivion_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(async_get, m)?)?;
    m.add_function(wrap_pyfunction!(post, m)?)?;
    m.add_function(wrap_pyfunction!(async_post, m)?)?;
    m.add_function(wrap_pyfunction!(put, m)?)?;
    m.add_function(wrap_pyfunction!(async_put, m)?)?;
    Ok(())
}
