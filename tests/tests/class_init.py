class Test:
    def __init__(self, x, y):
        self.x = x + 2
        self.y = y + 2

    def calc(self):
        return self.x + self.y

t = Test(2, 3)
assert t.calc() == 9
