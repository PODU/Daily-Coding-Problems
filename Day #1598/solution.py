# Day 1598: Round floats preserving sum: floor all, then round up the d elements with
# largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
import math


def round_preserve(x):
    n = len(x)
    y = [int(math.floor(v)) for v in x]
    floor_sum = sum(y)
    target = round(sum(x))
    d = target - floor_sum
    order = sorted(range(n), key=lambda i: x[i] - math.floor(x[i]), reverse=True)
    for i in range(d):
        y[order[i]] += 1
    return y


if __name__ == "__main__":
    x = [1.3, 2.3, 4.4]
    y = round_preserve(x)
    print(y)
    diff = sum(abs(x[i] - y[i]) for i in range(len(x)))
    print("abs diff = %.1f" % diff)
