# Day 62: Count lattice paths in N x M grid via combinatorics C(n+m-2, n-1).
# Time O(min(n,m)), Space O(1).
from math import comb


def paths(n, m):
    return comb((n - 1) + (m - 1), n - 1)


if __name__ == "__main__":
    print(f"2 by 2 matrix -> {paths(2, 2)}")
    print(f"5 by 5 matrix -> {paths(5, 5)}")
