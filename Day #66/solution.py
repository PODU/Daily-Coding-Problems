# Day 66: Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.
import random

rng = random.Random(12345)


def toss_biased():  # simulated bias p = 0.3 for 1
    return 1 if rng.random() < 0.3 else 0


def toss_fair():
    while True:
        a = toss_biased()
        b = toss_biased()
        if a == 0 and b == 1:
            return 0
        if a == 1 and b == 0:
            return 1


if __name__ == "__main__":
    trials = 100000
    ones = sum(toss_fair() for _ in range(trials))
    frac = ones / trials
    assert 0.48 < frac < 0.52
    print("Fair coin ~0.5")
