# Day 1845: Index of nearest larger value (by array distance) via outward expansion.
# Time O(N) per query, Space O(1). Returns None if none exists.


def nearest_larger(a, i):
    n = len(a)
    for d in range(1, n):
        if i - d >= 0 and a[i - d] > a[i]:
            return i - d
        if i + d < n and a[i + d] > a[i]:
            return i + d
    return None


def main():
    print(nearest_larger([4, 1, 3, 5, 6], 0))  # 3


if __name__ == "__main__":
    main()
