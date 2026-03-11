# Day 1191: Approximate median: take a constant-size random sample and return its median.
# Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
import random


def approx_median(a):
    N = len(a)
    k = min(N, 31)
    rng = random.Random(42)  # fixed seed for reproducibility
    sample = [a[rng.randrange(N)] for _ in range(k)]
    sample.sort()
    med = sample[len(sample) // 2]
    rank = sum(1 for x in a if x <= med)
    return med, rank


if __name__ == "__main__":
    a = list(range(1, 101))
    N = len(a)
    v, rank = approx_median(a)
    print(f"Approximate median: {v} (rank {rank} within [{N // 4}, {3 * N // 4}])")
