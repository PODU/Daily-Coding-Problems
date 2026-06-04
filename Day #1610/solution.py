# Day 1610: Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
# Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.
import random


def reservoir_sample(stream, seed):
    rng = random.Random(seed)
    kept = None
    for i, value in enumerate(stream):  # i is 0-indexed
        # for the (i+1)-th element keep with prob 1/(i+1)
        if rng.randrange(i + 1) == 0:
            kept = value
    return kept


def main():
    stream = list(range(1, 11))
    selected = reservoir_sample(stream, 42)
    print(f"Selected: {selected}")


if __name__ == "__main__":
    main()
