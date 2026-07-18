# Day 1832: Expected rounds until one coin remains. Each round a coin survives w.p. 1/2.
# DP recurrence: E(n)*(2^n-1) = 2^n + sum_{k=2..n-1} C(n,k) E(k); E(0)=E(1)=0. O(n^2).
from math import comb


def expected_rounds(n):
    if n <= 1:
        return 0.0
    E = [0.0] * (n + 1)
    for m in range(2, n + 1):
        pm = 2.0 ** m
        s = pm + sum(comb(m, k) * E[k] for k in range(2, m))
        E[m] = s / (pm - 1.0)
    return E[n]


if __name__ == "__main__":
    n = 4
    print(expected_rounds(n))  # ~2.05714 rounds
