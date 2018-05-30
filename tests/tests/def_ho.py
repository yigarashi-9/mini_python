def mult(n, m):
    def rec(i):
        if i == n:
            return 0
        else:
            return m + rec(i + 1)
    return rec(0)

def f(g):
    return g(10, 5)

assert f(mult) == 50
