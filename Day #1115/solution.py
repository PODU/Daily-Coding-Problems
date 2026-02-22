# Day 1115: Day 1115 - Uniform random in [0, n) excluding list l
# Approach: remap the |E| excluded slots below m=n-|E| to available high slots,
# then sample once in [0, m). Time: O(|E|) prep, O(1)/sample. Space: O(|E|).
import random


class Sampler:
    def __init__(self, n, l):
        self.n = n
        excluded = {x for x in l if 0 <= x < n}
        self.m = n - len(excluded)
        available = [v for v in range(self.m, n) if v not in excluded]
        self.mapping = {}
        ai = 0
        for e in excluded:
            if e < self.m:
                self.mapping[e] = available[ai]
                ai += 1

    def sample(self):
        r = random.randrange(self.m)
        return self.mapping.get(r, r)


if __name__ == "__main__":
    n, l = 10, [2, 5, 7]
    s = Sampler(n, l)
    seen = set(s.sample() for _ in range(2000))
    print(f"n={n}, excluded={l}")
    print("Sampled valid numbers:", sorted(seen))
    # n=10, excluded=[2, 5, 7]
    # Sampled valid numbers: [0, 1, 3, 4, 6, 8, 9]
