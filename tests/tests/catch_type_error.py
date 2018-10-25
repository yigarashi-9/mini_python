def f():
    return 42

x = 1

try:
    f + 42
except:
    x = 2

assert x == 2
