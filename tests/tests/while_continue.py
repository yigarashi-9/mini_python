x = 0
y = 0
while x < 10:
    x = x + 1
    if x < 6:
        continue
    else:
        y = y + 1
assert y == 5
