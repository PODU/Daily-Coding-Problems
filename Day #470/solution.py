# Day 470: Nearest larger: expand outward from i, return first index with greater value.
# Time: O(n), Space: O(1).
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
    r = nearest_larger(a, 0)
    print("null" if r is None else r)
