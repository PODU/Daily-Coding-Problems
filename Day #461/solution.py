# Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
# Multiplicative binomial. Time O(min(N,M)), Space O(1).
from math import comb


def count_paths(n, m):
    return comb((n - 1) + (m - 1), n - 1)


if __name__ == "__main__":
    print(count_paths(2, 2))  # 2
    print(count_paths(5, 5))  # 70
