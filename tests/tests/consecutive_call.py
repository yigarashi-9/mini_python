def f1():
    return f2

def f2():
    return f3

def f3():
    return 42

assert f1()()() == 42
