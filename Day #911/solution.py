# Day 911: Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
import random


def reservoir_sample(stream, rng):
    pick = None
    for i, x in enumerate(stream, start=1):  # 1-indexed
        if rng.randint(1, i) == 1:  # replace with probability 1/i
            pick = x
    return pick


if __name__ == "__main__":
    rng = random.Random(12345)
    stream = list(range(10))  # 0..9

    print("Sampled element:", reservoir_sample(stream, rng))

    trials, n = 100000, 10
    freq = [0] * n
    for _ in range(trials):
        freq[reservoir_sample(stream, rng)] += 1
    print(f"Approx frequencies over {trials} trials (expect ~{1/n:.4f} each):")
    for v in range(n):
        print(f"  {v}: {freq[v] / trials:.4f}")
    print("Distribution is ~uniform.")
