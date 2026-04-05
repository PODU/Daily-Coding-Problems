# Day 1304: Uniformly sample an integer in [0, n) not in list l.
# Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
# Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
import random
from bisect import bisect_right


class RandExcluder:
    def __init__(self, n, l, seed=42):
        self.n = n
        self.excl = sorted({x for x in l if 0 <= x < n})
        self.rng = random.Random(seed)

    def next(self):
        count = self.n - len(self.excl)
        res = self.rng.randrange(count)
        for e in self.excl:
            if e <= res:
                res += 1
            else:
                break
        return res


if __name__ == "__main__":
    r = RandExcluder(5, [1, 3])
    seen = sorted({r.next() for _ in range(1000)})
    print(seen)  # [0, 2, 4]
