# Day 1734: Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
# Time: O(log n), Space: O(1).
def fixed_point(a):
    lo, hi, ans = 0, len(a) - 1, -1
    while lo <= hi:
        mid = (lo + hi) // 2
        if a[mid] == mid:
            ans = mid
            hi = mid - 1
        elif a[mid] < mid:
            lo = mid + 1
        else:
            hi = mid - 1
    return ans if ans != -1 else False


if __name__ == "__main__":
    print(fixed_point([-6, 0, 2, 40]))
    print(fixed_point([1, 5, 7, 8]))
