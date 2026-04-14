# Day 1359: Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
# Demo uses a portable 64-bit LCG seeded with 1 so output is deterministic across languages -> 7.

MASK = (1 << 64) - 1
A = 6364136223846793005
C = 1442695040888963407


def reservoir_pick(stream, seed):
    state = seed & MASK
    pick = None
    for i, x in enumerate(stream, start=1):
        state = (state * A + C) & MASK   # advance LCG (mod 2^64)
        if state % i == 0:               # replace with probability 1/i
            pick = x
    return pick


if __name__ == "__main__":
    stream = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    print("Selected:", reservoir_pick(stream, 1))  # fixed seed = 1
