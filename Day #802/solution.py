# Day 802: Sample a number per given probability distribution.
# Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).
import random
import bisect


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
        i = bisect.bisect_left(self.cdf, u)
        return self.nums[min(i, len(self.nums) - 1)]


if __name__ == "__main__":
    numbers = [1, 2, 3, 4]
    probs = [0.1, 0.5, 0.2, 0.2]
    s = WeightedSampler(numbers, probs, seed=42)
    trials = 100000
    count = {n: 0 for n in numbers}
    for _ in range(trials):
        count[s.sample()] += 1
    for n in numbers:
        print(f"{n}: {count[n] / trials:.2f}")
    # ~ 1:0.10  2:0.50  3:0.20  4:0.20
