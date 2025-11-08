# Day 569: Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
# Time: O(N) with <2N comparisons. Space: O(1).

def min_max(a):
    cmps = 0
    n = len(a)
    if n % 2 == 1:
        mn = mx = a[0]
        i = 1
    else:
        cmps += 1
        if a[0] < a[1]:
            mn, mx = a[0], a[1]
        else:
            mn, mx = a[1], a[0]
        i = 2
    while i + 1 < n:
        cmps += 1
        if a[i] < a[i + 1]:
            lo, hi = a[i], a[i + 1]
        else:
            lo, hi = a[i + 1], a[i]
        cmps += 1
        if lo < mn:
            mn = lo
        cmps += 1
        if hi > mx:
            mx = hi
        i += 2
    return mn, mx, cmps


if __name__ == "__main__":
    arr = [7, 2, 9, 4, 1, 8, 3]
    mn, mx, _ = min_max(arr)
    print(f"min={mn} max={mx}")
