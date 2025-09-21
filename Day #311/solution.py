# Day 311: Find a peak (greater than both neighbors) when ends are the lowest.
# Binary search toward the rising side. O(log N).
def find_peak(a):
    lo, hi = 0, len(a) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] < a[mid + 1]:
            lo = mid + 1
        else:
            hi = mid
    return lo


if __name__ == "__main__":
    a = [1, 3, 5, 7, 6, 4, 2]
    p = find_peak(a)
    print("index {} value {}".format(p, a[p]))  # index 3 value 7
