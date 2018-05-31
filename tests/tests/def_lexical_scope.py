def f():
    x = 1
    def g():
        return x + 41
    return g

x = 10
assert (f())() == 42
