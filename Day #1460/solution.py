# Day 1460: Fixed point (arr[i]==i) in sorted distinct array via binary search.
# Time O(log n), Space O(1).


def fixed_point(arr):
    lo, hi = 0, len(arr) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if arr[mid] == mid:
            return mid
        elif arr[mid] < mid:
            lo = mid + 1
        else:
            hi = mid - 1
    return False


def run(arr):
    r = fixed_point(arr)
    print(r)


def main():
    run([-6, 0, 2, 40])
    run([1, 5, 7, 8])


if __name__ == '__main__':
    main()
