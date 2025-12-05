# Day 695: Uniform random in [0, n-1] excluding values in list l.
# Approach: precompute the allowed values once, then pick a uniform index.
# Preprocess O(n), each draw O(1).
import random


class RandExcluder:
    def __init__(self, n, l, seed=42):
        bad = set(l)
        self.allowed = [x for x in range(n) if x not in bad]
        self.rng = random.Random(seed)

    def next(self):
        return self.rng.choice(self.allowed)


if __name__ == "__main__":
    r = RandExcluder(10, [2, 4, 6, 8])
    print("sample:", [r.next() for _ in range(8)])
    print("(all values are in [0,9] and never 2,4,6,8)")
