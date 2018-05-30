def rec(i, n):
    if i == n:
        return 0
    else:
        return 1 + rec(i + 1, n)
assert rec(0, 10) == 10
