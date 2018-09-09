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
