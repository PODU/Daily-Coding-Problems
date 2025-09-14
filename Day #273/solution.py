# Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
# Time O(log N), Space O(1). Returns index or False.


def fixed_point(a):
    lo, hi = 0, len(a) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if a[mid] == mid:
            return mid
        elif a[mid] < mid:
            lo = mid + 1
        else:
            hi = mid - 1
    return False


if __name__ == "__main__":
    print(fixed_point([-6, 0, 2, 40]))  # 2
    print(fixed_point([1, 5, 7, 8]))    # False
