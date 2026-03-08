# Day 1181: Find the minimum in a rotated sorted array (no duplicates).
# Binary search: compare mid to the right end to discard the sorted half.
# Time O(log N), Space O(1).


def find_min(a):
    lo, hi = 0, len(a) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] > a[hi]:
            lo = mid + 1
        else:
            hi = mid
    return a[lo]


if __name__ == "__main__":
    print(find_min([5, 7, 10, 3, 4]))  # 3
