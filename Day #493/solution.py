# Day 493: Sample from a discrete distribution.
# Precompute cumulative probabilities; binary-search a uniform r in [0,1).
# Time: O(n) preprocessing, O(log n) per sample. Space: O(n).
import bisect
import random
from collections import Counter


class DiscreteSampler:
    def __init__(self, numbers, probs):
        self.numbers = numbers
        self.cdf = []
        acc = 0.0
        for p in probs:
            acc += p
            self.cdf.append(acc)

    def sample(self, r):
        # first index whose cdf > r
        idx = bisect.bisect_right(self.cdf, r)
        return self.numbers[min(idx, len(self.numbers) - 1)]


if __name__ == "__main__":
    numbers = [1, 2, 3, 4]
    probs = [0.1, 0.5, 0.2, 0.2]
    sampler = DiscreteSampler(numbers, probs)

    rng = random.Random(42)
    N = 100000
    counts = Counter(sampler.sample(rng.random()) for _ in range(N))
    for n in numbers:
        print(f"{n}: {counts[n] / N:.3f}")
