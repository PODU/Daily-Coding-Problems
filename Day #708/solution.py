# Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
# Since values are distinct integers, a[i]-i is monotonic. Time O(log n).

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


def report(a):
    r = fixed_point(a)
    print(r if r is not False else "False")


if __name__ == "__main__":
    report([-6, 0, 2, 40])
    report([1, 5, 7, 8])
