# Day 1637: Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
# Time: O(log N), Space: O(1).

def midpoint(lo, hi):
    i, j = lo, hi
    while i < j:
        i += 1
        j -= 1
    return j  # floor((lo+hi)/2) using only +/-


def contains(arr, x):
    lo, hi = 0, len(arr) - 1
    while lo <= hi:
        mid = midpoint(lo, hi)
        if arr[mid] == x:
            return True
        elif arr[mid] < x:
            lo = mid + 1
        else:
            hi = mid - 1
    return False


def main():
    arr = [1, 3, 5, 7, 9, 11, 13]
    print("true" if contains(arr, 7) else "false")
    print("true" if contains(arr, 8) else "false")


if __name__ == "__main__":
    main()
