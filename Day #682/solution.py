# Day 682: add_subtract: true currying via int subclass that is also callable.
# Each call alternates +/-. Time/space O(n) in number of args.

class _AddSub(int):
    def __new__(cls, value, sign=1):
        obj = super().__new__(cls, value)
        obj._sign = sign  # sign applied to the NEXT argument
        return obj

    def __call__(self, x):
        return _AddSub(int(self) + self._sign * x, -self._sign)


def add_subtract(first):
    # first arg kept as-is, next added (+), then subtracted (-), then +, ...
    return _AddSub(first, 1)


if __name__ == "__main__":
    print(int(add_subtract(7)))                 # 7
    print(int(add_subtract(1)(2)(3)))           # 0
    print(int(add_subtract(-5)(10)(3)(9)))      # 11
