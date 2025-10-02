# Day 355: Round floats to ints keeping sum == round(total) with minimal total abs diff.
# Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
import math


def round_preserving_sum(x):
    n = len(x)
    y = [math.floor(v) for v in x]
    floor_sum = sum(y)
    target = round(sum(x))
    need = target - floor_sum
    # indices with largest fractional part first, skip integers (frac 0)
    order = sorted(range(n), key=lambda i: x[i] - math.floor(x[i]), reverse=True)
    for i in order:
        if need <= 0:
            break
        if x[i] - math.floor(x[i]) > 0:
            y[i] += 1
            need -= 1
    return [int(v) for v in y]


def main():
    x = [1.3, 2.3, 4.4]
    print(round_preserving_sum(x))


if __name__ == "__main__":
    main()
