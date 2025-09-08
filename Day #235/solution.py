# Day 235: Min & Max in ~3N/2 comparisons: process elements in pairs, compare the pair first,
# then smaller vs min and larger vs max. Time: O(N), Space: O(1). Comparisons ~ 3*ceil(N/2)-2.


def min_max(a):
    n = len(a)
    if n % 2 == 0:
        mn, mx = (a[0], a[1]) if a[0] < a[1] else (a[1], a[0])
        i = 2
    else:
        mn = mx = a[0]
        i = 1
    while i < n:
        x, y = a[i], a[i + 1]
        if x < y:
            mn = min(mn, x)
            mx = max(mx, y)
        else:
            mn = min(mn, y)
            mx = max(mx, x)
        i += 2
    return mn, mx


if __name__ == "__main__":
    a = [3, 5, 1, 2, 4, 8, 7]
    mn, mx = min_max(a)
    print(f"min={mn} max={mx}")  # min=1 max=8
