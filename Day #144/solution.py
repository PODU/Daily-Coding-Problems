# Day 144: Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.


def nearest_larger(a, i):
    n = len(a)
    for d in range(1, n):
        if i - d >= 0 and a[i - d] > a[i]:
            return i - d
        if i + d < n and a[i + d] > a[i]:
            return i + d
    return None


if __name__ == "__main__":
    a = [4, 1, 3, 5, 6]
    print(nearest_larger(a, 0))  # 3
