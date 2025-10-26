# Day 495: Reservoir sampling (size 1) from a stream of unknown length.
# For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
# Time: O(n) per pass, Space: O(1).
import random
from collections import Counter


def reservoir_sample(stream, rng):
    """Pick one element uniformly from an iterable of unknown length without storing it."""
    chosen = None
    for count, x in enumerate(stream, start=1):
        if rng.random() < 1.0 / count:
            chosen = x
    return chosen


if __name__ == "__main__":
    rng = random.Random(42)
    TRIALS = 100000
    counts = Counter(reservoir_sample(iter(range(1, 11)), rng) for _ in range(TRIALS))

    print("Empirical selection frequency per element (~0.100 each):")
    for v in range(1, 11):
        print(f"{v}: {counts[v] / TRIALS:.3f}")
    print("One sampled value:", reservoir_sample(iter(range(1, 11)), rng))
