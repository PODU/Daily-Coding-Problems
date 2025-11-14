# Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
# DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n).
from math import comb


def expected_rounds(n):
    if n <= 1:
        return 0.0
    f = [0.0] * (n + 1)
    for m in range(2, n + 1):
        half = 0.5 ** m
        s = sum(comb(m, k) * f[k] for k in range(m))
        f[m] = (1.0 + half * s) / (1.0 - half)
    return f[n]


def main():
    n = 4
    print(f"{expected_rounds(n):.4f}")  # ~2.0571


if __name__ == '__main__':
    main()
