use findgrep::{findgrep, FindResult, GrepResult};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn grep_result_to_py(grep_result: &GrepResult, py: Python) -> PyObject {
    let dict = PyDict::new(py);
    dict.set_item("pattern", grep_result.pattern.clone())
        .unwrap();
    dict.set_item("matching_text", grep_result.matching_text.clone())
        .unwrap();
    dict.set_item("matching_line", grep_result.matching_line.clone())
        .unwrap();
    dict.set_item("lineno", grep_result.lineno).unwrap();
    dict.into()
}

fn find_result_to_py(find_result: &FindResult, py: Python) -> PyObject {
    let dict = PyDict::new(py);
    dict.set_item("path", find_result.path.as_path().to_str().clone())
        .unwrap();
    dict.set_item("path_type", find_result.path_type.clone())
        .unwrap();
    let list = PyList::empty(py);
    for grep_result in &find_result.grep_results {
        if let Err(err) = list.append(grep_result_to_py(grep_result, py)) {
            println!("{}", err);
        }
    }
    dict.set_item("grep_results", list).unwrap();
    dict.into()
}

#[pyfunction]
fn findgrep_py(
    py: Python<'_>,
    path: String,
    threads: usize,
    ignore_hidden: bool,
    buffer_size: usize,
    log_errors: bool,
    only_files: bool,
    filter_by_grep: bool,
    match_patterns: Vec<String>,
    content_patterns: Vec<String>,
) -> PyResult<PyObject> {
    match findgrep(
        path,
        threads,
        ignore_hidden,
        buffer_size,
        log_errors,
        only_files,
        filter_by_grep,
        match_patterns,
        content_patterns,
    ) {
        Ok(find_results) => {
            let mut py_results = Vec::new();
            for find_result in find_results {
                py_results.push(find_result_to_py(&find_result, py));
            }
            let result_list = PyList::new(py, py_results)?;
            Ok(result_list.into())
        }
        Err(err) => {
            return Err(PyValueError::new_err(format!(
                "Error occurred in libwalk call: {}",
                err
            )));
        }
    }
}

#[pymodule]
fn pyfindgrep(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(findgrep_py, m)?)?;
    Ok(())
}
