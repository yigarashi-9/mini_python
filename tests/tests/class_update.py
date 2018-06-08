class Test:
    def initialize(self):
        self.x = 42

t = Test()
t.initialize()

def calc(self, n):
    return self.x + n

Test.calc = calc

assert t.calc(4) == 46
