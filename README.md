# Simple Rust module for Python Tutorial

### Intro

I'm using python 3.6 on Windows and couldn't get any of the tutorial to work that are available on the web.
 
My goal was to write a library written in Rust that can be loaded with python to speed up slow functions.

To list a few tutorials I wasn't happy about (maybe they are helpful for you though):

- https://github.com/rochacbruno/rust-python-example
- https://mycognosist.github.io/tutorial-rust-python-lib.html
- https://bheisler.github.io/post/calling-rust-in-python/
- https://github.com/alexcrichton/rust-ffi-examples
- https://doc.rust-lang.org/1.2.0/book/rust-inside-other-languages.html
- https://github.com/dgrunwald/rust-cpython

Major issues with them (for me only):
- Wrong file ending of the output file
- cffi / FFI didn't work
- How to convert Python types to Rust types and vice versa

In the end, the comment that helped me the most was https://github.com/dgrunwald/rust-cpython/issues/151

### Preparation

- Install Rust and Python
1) Open console / terminal and type `cargo new rustpylib` to create a new project / folder
2) Go to `rustpylib/src/` and rename `main.rs` to `lib.rs` and change its contents to 
    ```rust
    #[macro_use]
    extern crate cpython;
    
    use cpython::{PyObject, PyResult, PyInt, PyFloat, Python, PyList, PySequence, FromPyObject, ToPyObject};
    
    // required to call methods of python objects
    use cpython::ObjectProtocol;
    // see http://dgrunwald.github.io/rust-cpython/doc/cpython/struct.PyObject.html#impl-ObjectProtocol
    
    py_module_initializer!(rustpylib, initrustpylib, PyInit_rustpylib, |py, m| {
        try!(m.add(py, "__doc__", "This module is implemented in Rust."));
        try!(m.add(py, "integer_test", py_fn!(py, integer_test_py(my_py_number: u32))));
        try!(m.add(py, "string_test", py_fn!(py, string_test_py(my_py_string: String))));
        try!(m.add(py, "list_test", py_fn!(py, list_test_py(my_list: PyObject))));
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
    ```
3) Change contents of `rustpylib/Cargo.toml` to 
    ```
    [package]
    name = "rustpylib"
    version = "0.1.0"
    authors = ["BurnySc2 <gamingburny@gmail.com>"]
    
    
    [dependencies.cpython]
    version = "0.2"
    features = ["extension-module"]
    
    [lib]
    name = "rustpylib"
    crate-type = ["cdylib"]
    ```

4) Compile the rust library by going into `rustpylib/` and type `cargo build --release`

    A new folder will be created in `rustpylib/target/release/`
    
    The important resulting file is `rustpylib/target/release/rustpylib.dll` (or `.so` on linux)
    
    If you encounter into compile errors, try out the nightly Rust build
    
    - `rustup install nightly`
    
    - `rustup override add nightly`

5) On Linux: skip this step
 
    On Windows: rename `rustpylib/target/release/rustpylib.dll` to `rustpylib/target/release/rustpylib.pyd` 
    
    On MAC OS: rename `rustpylib/target/release/rustpylib.dylib` to `rustpylib/target/release/rustpylib.so`
    
6) Create an example Python file `rustpylib/target/release/pythontest.py` with content
    ```python
    import time
    import rustpylib
    
    def testInt():
        t0 = time.time()
        result = rustpylib.integer_test(5)
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    def testString():
        t0 = time.time()
        result = rustpylib.string_test("My python string ")
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    def testList():
        t0 = time.time()
        result = rustpylib.list_test([1, 2, 7])
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    class Point:
        def __init__(self, x, y):
            self.x = x
            self.y = y
        def distance_to(self, other):
            return ((other.x - self.x)**2 + (other.y - self.y)**2)**0.5
    
    def testClass():
        t0 = time.time()
        p1 = Point(2, 4)
        p2 = Point(5, 8)
        result = rustpylib.class_test(p1, p2)
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    if __name__ == "__main__":
        testInt()
        testString()
        testList()
        testClass()
    ```

- Output after executing the `pythontest.py` file should be:
    ```
    C:\Python36\python.exe C:/rustpylib/target/release/pythontest.py
    Time required: 0.0, output: 10, output type: <class 'int'>
    Time required: 0.0, output: My python string My rust string, output type: <class 'str'>
    Time required: 0.0, output: [1, 2, 3, 4, 1, 2, 7], output type: <class 'list'>
    From Rust in class_test_py
    Input p1: (2, 4)
    Input p2: (5, 8)
    Rust result: 5
    Time required: 0.0, output: [5.0, 5.0], output type: <class 'list'>
    ```

# More information

http://dgrunwald.github.io/rust-cpython/doc/cpython/#structs

https://github.com/dgrunwald/rust-cpython
    
## Happy coding!