# Simple Rust module for Python Tutorial

### Intro

I'm using python 3.6 on Windows and couldn't get any of the tutorial to work that are available on the web.
 
My goal was to write a library written in Rust that can be loaded with python to speed up slow functions.

To list a few tutorials I wasn't happy about:

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

5) On Linux: skip this step
 
    On Windows: rename `rustpylib/target/release/rustpylib.dll` to `rustpylib/target/release/rustpylib.pyd` 
    
    On MAC OS: rename `rustpylib/target/release/rustpylib.dylib` to `rustpylib/target/release/rustpylib.so`
    
6) Create an example Python file `rustpylib/target/release/pythontest.py` with content
    ```python
    import time
    import rustpylib
    
    def testInt():
        t0 = time.time()
        result = rustpylib.integer_test()
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    def testString():
        t0 = time.time()
        result = rustpylib.string_test()
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    def testVec():
        t0 = time.time()
        result = rustpylib.list_test()
        t1 = time.time()
        print("Time required: {}, output: {}, output type: {}".format(t1-t0, result, type(result)))
    
    if __name__ == "__main__":
        testInt()
        testString()
        testVec()
    ```

- Output after executing the `pythontest.py` file should be:
    ```
    C:\Python36\python.exe C:/rustpylib/target/release/pythontest.py
    Time required: 0.0, output: 5, output type: <class 'int'>
    Time required: 0.0, output: I think, this is working just fine., output type: <class 'str'>
    Time required: 0.0, output: [1, 2, 3, 4], output type: <class 'list'>
    ```
- Happy coding!