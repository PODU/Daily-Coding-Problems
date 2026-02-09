# Day 1051: Search in rotated sorted array via modified binary search.
# Time: O(log n), Space: O(1).
from typing import List, Optional


def search_rotated(a: List[int], target: int) -> Optional[int]:
    lo, hi = 0, len(a) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if a[mid] == target:
            return mid
        if a[lo] <= a[mid]:               # left half sorted
            if a[lo] <= target < a[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:                             # right half sorted
            if a[mid] < target <= a[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return None


if __name__ == "__main__":
    arr = [13, 18, 25, 2, 8, 10]
    print(search_rotated(arr, 8))  # 4
