# Day 1543: Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
# unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
import random


def estimate_pi(samples, seed=42):
    rng = random.Random(seed)
    inside = 0
    for _ in range(samples):
        x, y = rng.random(), rng.random()
        if x * x + y * y <= 1.0:
            inside += 1
    return 4.0 * inside / samples


if __name__ == "__main__":
    print(f"{estimate_pi(2_000_000):.3f}")
