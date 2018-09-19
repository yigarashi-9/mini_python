l1 = []
l2 = [1 + 2]
l3 = [3, [1, 2, 3], "abc"]

if l1:
    x = 0
else:
    x = 1

assert x + l2[0] + l3[1][0] == 5
