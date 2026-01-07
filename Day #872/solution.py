# Day 872: Approximate median: take a small random sample (size s) and return its median.
# Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).
import random


def approx_median(a, sample_size, rng):
    n = len(a)
    s = min(sample_size, n)
    sample = [a[rng.randrange(n)] for _ in range(s)]
    sample.sort()
    return sample[s // 2]


if __name__ == "__main__":
    rng = random.Random(42)
    n = 1000
    a = list(range(n))
    rng.shuffle(a)
    m = approx_median(a, 51, rng)
    rank = sum(1 for x in a if x < m)
    print("approx median =", m)
    ok = n // 4 <= rank <= 3 * n // 4
    print(f"rank {rank} in [{n // 4}, {3 * n // 4}]: {str(ok).lower()}")
