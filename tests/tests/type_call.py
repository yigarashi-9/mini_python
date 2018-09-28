x = 0
n = 3
b = True

if type(n) == int:
    if type(b) == bool:
        if type(type(n)) == type:
            x = 1
        else:
            x = 10000
    else:
        x = 10000
else:
    x = 100000

assert x == 1
