# Day 15: Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
# Time: O(n) single pass, Space: O(1).
import random


def pick_random(stream, rng):
    chosen = None
    for count, x in enumerate(stream, start=1):
        if rng.randrange(count) == 0:
            chosen = x
    return chosen


if __name__ == "__main__":
    rng = random.Random(42)
    stream = [10, 20, 30, 40, 50]
    freq = {}
    for _ in range(100000):
        s = pick_random(stream, rng)
        freq[s] = freq.get(s, 0) + 1
    print("One sample:", pick_random(stream, rng))
    print("Approx frequencies over 100000 trials:")
    for k in sorted(freq):
        print(f"  {k}: {freq[k] / 100000:.3f}")
