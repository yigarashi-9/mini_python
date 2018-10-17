x = 1

try:
    raise Exception()
except:
    x = 2

assert x == 2
