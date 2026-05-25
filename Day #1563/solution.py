# Day 1563: Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).


def count_paths(n, m):
    total = n + m - 2
    k = min(n - 1, m - 1)
    res = 1
    for i in range(1, k + 1):
        res = res * (total - k + i) // i
    return res


def main():
    print(count_paths(2, 2))


if __name__ == "__main__":
    main()
