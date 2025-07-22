# Day 14: Estimate pi via Monte Carlo: fraction of random points in unit circle * 4.
# Time: O(samples), Space: O(1). Seeded for reproducible 3-decimal output.
import random


def estimate_pi(samples):
    rng = random.Random(12345)
    inside = 0
    for _ in range(samples):
        x, y = rng.random(), rng.random()
        if x * x + y * y <= 1.0:
            inside += 1
    return 4.0 * inside / samples


if __name__ == "__main__":
    print(f"{estimate_pi(2000000):.3f}")
