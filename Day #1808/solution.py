# Day 1808: Curried add_subtract: each call alternates +/- on the running total.
# Subclass int so the result is both callable (chainable) and usable as a number.
# Time: O(1) per call. Space: O(1).


class AddSub(int):
    def __new__(cls, value, sign):
        obj = super().__new__(cls, value)
        obj.sign = sign
        return obj

    def __call__(self, y):
        return AddSub(int(self) + self.sign * y, -self.sign)


def add_subtract(x):
    return AddSub(x, 1)


if __name__ == "__main__":
    print(add_subtract(7))                 # 7
    print(add_subtract(1)(2)(3))           # 0
    print(add_subtract(-5)(10)(3)(9))      # 11
