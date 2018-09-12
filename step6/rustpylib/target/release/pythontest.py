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
