# Day 1063: Find a peak in a rotated sorted array of distinct values.
# Approach: binary search; if a[mid] < a[mid+1] go right else left. Time O(log n), Space O(1).


def find_peak(a):
    lo, hi = 0, len(a) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] < a[mid + 1]:
            lo = mid + 1
        else:
            hi = mid
    return lo  # index of the peak


if __name__ == "__main__":
    a = [3, 4, 5, 1, 2]
    idx = find_peak(a)
    print(a[idx])  # 5
