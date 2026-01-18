# Day 915: Floor all; round up the `deficit` elements with largest fractional parts to match round(sum). O(n log n) time, O(n) space.
import math


def round_to_sum(x):
    n = len(x)
    y = [int(math.floor(v)) for v in x]
    floor_sum = sum(y)
    total = sum(x)
    target = round(total)
    deficit = target - floor_sum
    # indices sorted by largest fractional part
    order = sorted(range(n), key=lambda i: x[i] - math.floor(x[i]), reverse=True)
    for k in range(int(deficit)):
        y[order[k]] += 1
    return y


if __name__ == "__main__":
    x = [1.3, 2.3, 4.4]
    print(round_to_sum(x))
