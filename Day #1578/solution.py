# Day 1578: Find min and max using ~3N/2 comparisons (pairwise method).
# Compare elements in pairs, then each pair contributes one compare to min and one to max.
# Time: O(N) with ceil(3N/2)-2 comparisons; Space: O(1).


def min_max(a):
    n = len(a)
    if n & 1:
        mn = mx = a[0]
        i = 1
    else:
        (mn, mx) = (a[0], a[1]) if a[0] < a[1] else (a[1], a[0])
        i = 2
    while i + 1 < n:
        (lo, hi) = (a[i], a[i + 1]) if a[i] < a[i + 1] else (a[i + 1], a[i])
        if lo < mn:
            mn = lo
        if hi > mx:
            mx = hi
        i += 2
    return mn, mx


if __name__ == "__main__":
    mn, mx = min_max([3, 5, 1, 2, 8, 7, 4])
    print(f"min={mn} max={mx}")  # min=1 max=8
