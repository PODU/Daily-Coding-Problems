# Day 558: Estimate Pi via Monte Carlo: sample random points in unit square, fraction inside
# quarter circle ~ pi/4. O(S) for S samples. Fixed seed for reproducible 3-decimal result.
import random


def estimate_pi(samples):
    rng = random.Random(12345)
    inside = 0
    for _ in range(samples):
        x = rng.random()
        y = rng.random()
        if x * x + y * y <= 1.0:
            inside += 1
    return 4.0 * inside / samples


if __name__ == "__main__":
    print(f"{estimate_pi(2000000):.3f}")  # ~3.142
