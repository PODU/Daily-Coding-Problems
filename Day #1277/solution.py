# Day 1277: Curried add_subtract — alternately + then - successive args.
# Return an int subclass that is also callable, so chaining and printing both work. O(1)/call.
class _AS(int):
    def __new__(cls, value, idx):
        obj = super().__new__(cls, value)
        obj.idx = idx
        return obj

    def __call__(self, x):
        nv = int(self) + x if self.idx % 2 == 1 else int(self) - x
        return _AS(nv, self.idx + 1)


def add_subtract(x):
    return _AS(x, 1)


if __name__ == "__main__":
    print(add_subtract(7))            # 7
    print(add_subtract(1)(2)(3))      # 0
    print(add_subtract(-5)(10)(3)(9)) # 11
