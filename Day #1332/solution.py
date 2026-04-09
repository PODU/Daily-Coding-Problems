# Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
# Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).
import math


def round_preserve_sum(x):
    n = len(x)
    target = int(math.floor(sum(x) + 0.5))
    floors = [math.floor(v) for v in x]
    base = sum(floors)
    k = target - base  # how many to round up
    fracs = sorted(range(n), key=lambda i: x[i] - floors[i], reverse=True)
    y = floors[:]
    for i in range(k):
        y[fracs[i]] += 1
    return y


if __name__ == "__main__":
    print(round_preserve_sum([1.3, 2.3, 4.4]))  # [1, 2, 5]
