# Day 1839: Minimum of a rotated sorted array (no duplicates) via binary search.
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


def main():
    print(find_min([5, 7, 10, 3, 4]))


if __name__ == "__main__":
    main()
