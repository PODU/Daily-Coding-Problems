# Day 1746: Weighted random sampler.
# Approach: prefix-sum (CDF) of probabilities + binary search on uniform U[0,1).
# Build O(n), sample O(log n) time, O(n) space.
import random
import bisect
from collections import Counter


class WeightedSampler:
    def __init__(self, nums, probs, seed=42):
        self.nums = nums
        self.cdf = []
        acc = 0.0
        for p in probs:
            acc += p
            self.cdf.append(acc)
        self.rng = random.Random(seed)

    def sample(self):
        r = self.rng.random()
        idx = bisect.bisect_left(self.cdf, r)
        if idx >= len(self.nums):
            idx = len(self.nums) - 1
        return self.nums[idx]


def main():
    numbers = [1, 2, 3, 4]
    probs = [0.1, 0.5, 0.2, 0.2]
    s = WeightedSampler(numbers, probs)

    N = 1000000
    cnt = Counter(s.sample() for _ in range(N))
    print(f"Observed frequencies over {N} samples:")
    for x in numbers:
        print(f"{x}: {100.0 * cnt[x] / N:.1f}%")
    print("Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time")


if __name__ == "__main__":
    main()
