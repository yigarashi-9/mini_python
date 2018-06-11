d = {"a": 1 + 2, 3: 3 + 4}

def id(x):
    return x

id(d)["a"] = d[3] + 3
assert d["a"] == d[3] + 3
