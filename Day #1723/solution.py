# Day 1723: Search element in rotated sorted array.
# Modified binary search: one half is always sorted; decide which side to recurse.
# Time: O(log n), Space: O(1). Returns index, or None if absent.


def search(a, target):
    lo, hi = 0, len(a) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if a[mid] == target:
            return mid
        if a[lo] <= a[mid]:  # left half sorted
            if a[lo] <= target < a[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:  # right half sorted
            if a[mid] < target <= a[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return None


def main():
    a = [13, 18, 25, 2, 8, 10]
    print(search(a, 8))    # 4
    print(search(a, 99))   # None (not found)


if __name__ == "__main__":
    main()
