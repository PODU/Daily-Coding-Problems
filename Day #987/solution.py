# Day 987: Nearest larger number index.
# Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
# Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).


def nearest_larger(a, i):
    """Return index of nearest larger element, or None if none."""
    n = len(a)
    for d in range(1, n):
        l, r = i - d, i + d
        if l >= 0 and a[l] > a[i]:
            return l  # prefer left on ties
        if r < n and a[r] > a[i]:
            return r
    return None


if __name__ == "__main__":
    a = [4, 1, 3, 5, 6]
    idx = nearest_larger(a, 0)
    print("null" if idx is None else idx)  # expected 3
