#[macro_use]
extern crate cpython;

use cpython::{PyObject, PyResult, PyInt, Python, PyList, FromPyObject, ToPyObject};

py_module_initializer!(rustpylib, initrustpylib, PyInit_rustpylib, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "integer_test", py_fn!(py, integer_test_py())));
    try!(m.add(py, "string_test", py_fn!(py, string_test_py())));
    try!(m.add(py, "list_test", py_fn!(py, vec_test_py())));
    Ok(())
});

fn integer_test_py(py: Python) -> PyResult<PyInt> {
    let sum: u32 = 2 + 3;
    return Ok(sum.to_py_object(py));
}

fn string_test_py(py: Python) -> PyResult<String> {
    let test_string = "I think, this is working just fine.".to_string();
    return Ok(test_string);
}

fn vec_test_py(py: Python) -> PyResult<PyList> {
    let test_vec = vec![1, 2, 3, 4];
    Ok(test_vec.to_py_object(py))
}