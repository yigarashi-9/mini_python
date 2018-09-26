class A:
    def __init__(self, x):
        self.x = x


class B(A):
    def __add__(self, other):
        return self.x + other.x + 1


class C(A):
    pass = 1



def myadd(self, other):
    return self.x + other.x + 100


a1 = A(0)
a2 = A(0)
b1 = B(0)
b2 = B(0)
c1 = C(0)
c2 = C(0)

A.__add__ = myadd

assert (a1 + a2) + (b1 + b2) + (c1 + c2) == 201
