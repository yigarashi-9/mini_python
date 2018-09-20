class A:
    def f11(self):
        return 10000

    def f12(self):
        return 10000

    def f13(self):
        return 10000

    def f21(self):
        return 10000

    def f22(self):
        return 10000


class A11(A):
    def f11(self):
        return 1

    def f22(self):
        return 10000

class A12(A):
    def f12(self):
        return 1

    def f11(self):
        return 10000

class A13(A):
    def f11(self):
        return 10000

    def f12(self):
        return 10000

    def f13(self):
        return 1

    def f22(self):
        return 10000

class A21(A11, A12):
    def f21(self):
        return 1

class A22(A11, A13):
    def f21(self):
        return 10000

    def f22(self):
        return 1

class A3(A21, A22):
    def f3(self):
        return 1

o = A3()
assert o.f3() + o.f21() + o.f22() + o.f11() + o.f12() + o.f13() == 6
