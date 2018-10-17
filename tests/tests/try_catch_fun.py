def error():
    raise Exception

try:
    x = 1
    error()
except:
    x = 2

assert x == 2
