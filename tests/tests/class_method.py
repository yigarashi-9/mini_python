class Test:
    x = 2

    def f1(self, x):
        if 50 < x:
            return x
        else:
            return self.f2(self.x + x)

    def f2(self, x):
        if 50 < x:
            return x
        else:
            return self.f1(self.x + x)

t = Test()
assert t.f1(1) == 51
