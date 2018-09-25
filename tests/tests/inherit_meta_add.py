class A:
    def __add__(self, other):
        return self.x + other.x

class B(A):
    def __init__(self, x):
        self.x = x

b1 = B(4)
b2 = B(2)

assert b1 + b2 == 6
