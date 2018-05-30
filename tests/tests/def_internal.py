def mult(n, m):
    def rec(i):
        if i == n:
            return 0
        else:
            return m + rec(i + 1)
    return rec(0)
assert mult(5, 10) == 50
