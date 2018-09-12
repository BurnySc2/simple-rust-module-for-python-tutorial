#[macro_use]
extern crate cpython;

use cpython::{PyObject, PyResult, PyInt, PyFloat, Python, PyList, PySequence, FromPyObject, ToPyObject};

// required to call methods of python objects
use cpython::ObjectProtocol;
// see http://dgrunwald.github.io/rust-cpython/doc/cpython/struct.PyObject.html#impl-ObjectProtocol

use std::collections::HashSet;

py_module_initializer!(rustpylib, initrustpylib, PyInit_rustpylib, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "integer_test", py_fn!(py, integer_test_py(my_py_number: u32))));
    try!(m.add(py, "string_test", py_fn!(py, string_test_py(my_py_string: String))));
    try!(m.add(py, "list_test", py_fn!(py, list_test_py(my_list: PyObject))));
//    try!(m.add(py, "set_test", py_fn!(py, set_test_py(my_list: PyObject))));
    try!(m.add(py, "class_test", py_fn!(py, class_test_py(p1: PyObject, p2: PyObject))));
    Ok(())
});

fn integer_test_py(py: Python, my_py_number: u32) -> PyResult<PyInt> {
//    let my_rust_number = my_py_number.;
    let sum: u32 = 2 + 3 + my_py_number;
    return Ok(sum.to_py_object(py));
}

fn string_test_py(py: Python, my_py_string: String) -> PyResult<String> {
    let mut my_rust_string = my_py_string as String;
    let test_string = "My rust string".to_string();
    my_rust_string.push_str(&test_string);
//    // alternatively:
//    let test_string = "My rust string";
//    my_rust_string.push_str(test_string);
    return Ok(my_rust_string);
}

// https://github.com/dgrunwald/rust-cpython/issues/94
// how to call a python function? like list.append()
fn list_test_py(py: Python, my_list: PyObject) -> PyResult<PyList> {
    let mut test_vec = vec![1, 2, 3, 4];
    let my_rust_list = my_list.extract::<Vec<i32>>(py).unwrap();
    test_vec.extend(my_rust_list);
    let my_extended_list = test_vec.to_py_object(py);
    return Ok(my_extended_list);
}


struct RustPoint{
    x: i32,
    y: i32,
}

impl RustPoint {
    fn new(x: i32, y: i32) -> RustPoint {
        RustPoint { x: x, y: y }
    }

    // Calculate the distance between two Points
    fn rust_distance(&self, other: RustPoint) -> f32 {
        let (x1, y1) = (self.x, self.y);
        let (x2, y2) = (other.x, other.y);
        let distance = (((x2 - x1) as f32).powf(2.0) + ((y2 - y1) as f32).powf(2.0)).sqrt() as f32;
        return distance;
    }
}

fn class_test_py(py: Python, p1: PyObject, p2: PyObject) -> PyResult<PyList> {
    // convert python class Point to rust struct RustPoint
    let rust_p1 = RustPoint::new(
        p1.getattr(py, "x").unwrap().extract::<i32>(py).unwrap(),
        p1.getattr(py, "y").unwrap().extract::<i32>(py).unwrap()
    );
    let rust_p2 = RustPoint::new(
        p2.getattr(py, "x").unwrap().extract::<i32>(py).unwrap(),
        p2.getattr(py, "y").unwrap().extract::<i32>(py).unwrap()
    );
    println!("From Rust in class_test_py");
    println!("Input p1: ({}, {})", rust_p1.x, rust_p1.y);
    println!("Input p2: ({}, {})", rust_p2.x, rust_p2.y);
    let rust_result = rust_p1.rust_distance(rust_p2);
    println!("Rust result: {}", rust_result);

    // Use python call method on python object
    let py_result = p1.call_method(py, "distance_to", (p2,), None).unwrap();
    // Cast PyObject result to PyFloat
    let py_result_as_pyfloat = py_result.cast_into::<PyFloat>(py).unwrap();
    // Cast f32 float to PyFloat
    let rust_result_as_pyfloat = rust_result.to_py_object(py);
    // Put results in a vector
    let test_vec = vec![py_result_as_pyfloat, rust_result_as_pyfloat];
    // Convert vector to PyList
    Ok(test_vec.to_py_object(py))
}
