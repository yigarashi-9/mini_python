class A:
    def f(self):
        return 42

class B(A):
    def g(self):
        return 1

assert B().f() == 42
