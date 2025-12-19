# Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
# Process elements in pairs: compare the pair, then smaller vs min, larger vs max.


def min_max(a):
    n = len(a)
    if n % 2 == 0:
        mn, mx = (a[0], a[1]) if a[0] < a[1] else (a[1], a[0])
        i = 2
    else:
        mn = mx = a[0]
        i = 1
    while i < n:
        lo, hi = (a[i], a[i + 1]) if a[i] < a[i + 1] else (a[i + 1], a[i])
        if lo < mn:
            mn = lo
        if hi > mx:
            mx = hi
        i += 2
    return mn, mx


if __name__ == "__main__":
    mn, mx = min_max([3, 5, 1, 2, 4, 8, 7])
    print(f"min={mn} max={mx}")  # min=1 max=8
