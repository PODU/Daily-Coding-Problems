# Day 1593: rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
# Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.
import random

_rng = random.Random(12345)  # deterministic seed


def rand7() -> int:
    # uniform 1..7 using language RNG
    return _rng.randint(1, 7)


def rand5() -> int:
    # uniform 1..5 via rejection sampling
    while True:
        v = rand7()
        if v <= 5:
            return v


if __name__ == "__main__":
    N = 100000
    counts = [0] * 6
    for _ in range(N):
        counts[rand5()] += 1

    print("Distribution over %d samples:" % N)
    for v in range(1, 6):
        print("  %d: %d" % (v, counts[v]))

    print("First 10 samples:", " ".join(str(rand5()) for _ in range(10)))
