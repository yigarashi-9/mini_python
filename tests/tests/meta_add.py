class MyInt:
    def __init__(self, x):
        self.x = x

    def __add__(self, other):
        return self.x + other.x

i1 = MyInt(12)
i2 = MyInt(4)
assert i1 + i2 == 16
