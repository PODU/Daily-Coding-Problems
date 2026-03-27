# Day 1271: Implement rand5() from rand7() with uniform probability.
# Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
import random
from collections import Counter


def rand7() -> int:
    return random.randint(1, 7)


def rand5() -> int:
    v = rand7()
    while v > 5:        # reject 6, 7 -> uniform over 1..5
        v = rand7()
    return v


if __name__ == "__main__":
    trials = 100000
    dist = Counter(rand5() for _ in range(trials))
    print(f"Distribution over {trials} samples (expect ~{trials // 5} each):")
    for v in range(1, 6):
        print(f"{v}: {dist[v]}")
