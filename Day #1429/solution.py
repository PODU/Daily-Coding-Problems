# Day 1429: Find a peak element (greater than both neighbors) in O(log N).
# Approach: binary search; if a[mid] < a[mid+1] a peak lies right, else left/at mid.
# Time: O(log n), Space: O(1).

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
    print(a[find_peak(a)])  # 7
