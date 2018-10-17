try:
    x = 1
    while True:
        if x < 3:
            x = x + 1
        else:
            raise Exception
except:
    x = x + 1

assert x == 4
