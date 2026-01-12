# Day 884: rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).
import random

_rng = random.Random(12345)


def rand7():
    return _rng.randint(1, 7)


def rand5():
    x = rand7()
    while x > 5:
        x = rand7()
    return x


if __name__ == "__main__":
    counts = [0] * 6
    trials = 100000
    for _ in range(trials):
        counts[rand5()] += 1
    print(f"Distribution over {trials} samples:")
    for v in range(1, 6):
        print(f"{v}: {counts[v]}")
