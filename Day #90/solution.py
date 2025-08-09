# Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
# sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
import random


def random_excluding(n, l, rng=random):
    ex = sorted({v for v in l if 0 <= v < n})
    m = n - len(ex)
    if m <= 0:
        raise ValueError("no valid number")
    r = rng.randrange(m)
    for e in ex:
        if e <= r:
            r += 1
        else:
            break
    return r


if __name__ == "__main__":
    rng = random.Random(42)
    print(random_excluding(5, [1, 3], rng))  # sample from {0, 2, 4}
