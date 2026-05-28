# Day 1573: Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.

def find_peak(a):
    lo, hi = 0, len(a) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] < a[mid + 1]:
            lo = mid + 1
        else:
            hi = mid
    return a[lo]


if __name__ == "__main__":
    arr = [5, 7, 9, 3, 1]
    print(find_peak(arr))
