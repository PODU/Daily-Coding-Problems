# Day 890: Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
# Time: O(min(N,M)), Space: O(1).
def paths(n, m):
    total = n + m - 2
    k = min(n - 1, m - 1)
    res = 1
    for i in range(1, k + 1):
        res = res * (total - k + i) // i
    return res


if __name__ == "__main__":
    print(paths(2, 2))
    print(paths(5, 5))
