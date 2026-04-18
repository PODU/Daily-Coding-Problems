# Day 1383: Weighted sampling: build cumulative prefix array, draw u in [0,1), bisect to pick. O(n) prep, O(log n) per sample.
import random
import bisect


def main():
    numbers = [1, 2, 3, 4]
    probs = [0.1, 0.5, 0.2, 0.2]
    n = len(numbers)

    cum = []
    acc = 0.0
    for p in probs:
        acc += p
        cum.append(acc)

    rng = random.Random(42)
    N = 100000
    counts = [0] * n
    for _ in range(N):
        u = rng.random()
        idx = bisect.bisect_right(cum, u)
        if idx >= n:
            idx = n - 1
        counts[idx] += 1

    for i in range(n):
        print(f"{numbers[i]}: {counts[i] / N:.2f}")


if __name__ == "__main__":
    main()
