d = {True: True + False, False: False < True}

x = 0

if True == True:
    if False == False:
        x = d[True] + d[False] + (True == False) + 3
    else:
        x = 0
else:
    x = 0

assert x == 5
