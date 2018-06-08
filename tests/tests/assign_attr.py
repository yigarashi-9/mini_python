class Test:
    def test(self):
        h = Hoge()
        h.initialize()
        def generator():
            return h
        return generator

class Hoge:
    def initialize(self):
        self.x = 42

generator = Test().test()
generator().y = 2

assert generator().x + generator().y == 44
