# Day 410: Round floats so rounded sum is preserved with minimal total abs error.
# Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
import math


def round_to_sum(x):
    n = len(x)
    y = [math.floor(v) for v in x]
    floor_sum = sum(y)
    target = round(sum(x))
    R = target - floor_sum
    # indices sorted by fractional part descending
    idx = sorted(range(n), key=lambda i: x[i] - math.floor(x[i]), reverse=True)
    for i in range(min(R, n)):
        y[idx[i]] += 1
    return y


if __name__ == "__main__":
    x = [1.3, 2.3, 4.4]
    print(round_to_sum(x))
