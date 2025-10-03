# Day 363: Curried add_subtract that alternately adds/subtracts arguments.
# Return an int subclass that is also callable, so it prints as the running value
# yet can be chained. Time O(k) per chain of k args, Space O(1).


class _Result(int):
    def __new__(cls, value, count):
        obj = super().__new__(cls, value)
        obj.count = count
        return obj

    def __call__(self, x):
        # arg #1 adds, #2 subtracts, alternating.
        delta = x if self.count % 2 == 1 else -x
        return _Result(int(self) + delta, self.count + 1)


def add_subtract(first):
    return _Result(first, 1)


if __name__ == "__main__":
    print(add_subtract(7))
    print("1 + 2 - 3 ->", add_subtract(1)(2)(3))
    print("-5 + 10 - 3 + 9 ->", add_subtract(-5)(10)(3)(9))
