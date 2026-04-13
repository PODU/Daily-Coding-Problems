# Day 1351: Pairwise min/max: process pairs, compare smaller->min, larger->max -> ~3N/2 comparisons.
# Time: O(N) (~3N/2 comparisons), Space: O(1).
def min_max(a):
    n = len(a)
    if n % 2 == 0:
        (mn, mx) = (a[0], a[1]) if a[0] < a[1] else (a[1], a[0])
        i = 2
    else:
        mn = mx = a[0]
        i = 1
    while i + 1 <= n:
        if a[i] < a[i + 1]:
            small, large = a[i], a[i + 1]
        else:
            small, large = a[i + 1], a[i]
        if small < mn:
            mn = small
        if large > mx:
            mx = large
        i += 2
    return mn, mx


if __name__ == "__main__":
    a = [3, 5, 1, 2, 4, 8, 7]
    mn, mx = min_max(a)
    print(f"Min: {mn}, Max: {mx}")
