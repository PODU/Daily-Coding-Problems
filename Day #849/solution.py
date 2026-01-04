# Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
# Memoized chain lengths via an array. ~O(limit) amortized.
import sys


def steps(n):
    c = 0
    while n != 1:
        n = 3 * n + 1 if n & 1 else n // 2
        c += 1
    return c


def longest_under(limit):
    cache = [0] * (limit + 1)
    cache[1] = 1
    best_n, best_len = 1, 1
    for i in range(2, limit + 1):
        n, length = i, 0
        while n >= i or cache[n] == 0:
            n = 3 * n + 1 if n & 1 else n // 2
            length += 1
            if n == 1:
                break
        total = length + (cache[n] if n <= limit else 1)
        cache[i] = total
        if total > best_len:
            best_len, best_n = total, i
    return best_n, best_len


if __name__ == "__main__":
    print("27 reaches 1 in {} steps".format(steps(27)))  # 111
    n, ln = longest_under(1000000)
    print("Longest chain for n <= 1000000: n = {} (length {})".format(n, ln))  # 837799
