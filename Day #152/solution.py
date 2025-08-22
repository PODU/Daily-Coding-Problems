# Day 152: Weighted random sampling. Build cumulative distribution, draw u in
# [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
import random
from bisect import bisect_left


class WeightedSampler:
    def __init__(self, nums, probs, seed=None):
        self.nums = nums
        self.cdf = []
        acc = 0.0
        for p in probs:
            acc += p
            self.cdf.append(acc)
        self.rng = random.Random(seed)

    def sample(self):
        u = self.rng.random()
        idx = bisect_left(self.cdf, u)
        if idx >= len(self.nums):
            idx = len(self.nums) - 1
        return self.nums[idx]


if __name__ == "__main__":
    nums = [1, 2, 3, 4]
    probs = [0.1, 0.5, 0.2, 0.2]
    s = WeightedSampler(nums, probs, seed=42)
    N = 1_000_000
    counts = {n: 0 for n in nums}
    for _ in range(N):
        counts[s.sample()] += 1
    for n in nums:
        print(f"{n}: {100.0 * counts[n] / N:.2f}%")
