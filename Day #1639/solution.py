# Day 1639: Count inversions using modified merge sort (count cross-pairs during merge).
# Time: O(N log N), Space: O(N).

def _merge_count(a, tmp, lo, hi):
    if hi - lo <= 1:
        return 0
    mid = lo + (hi - lo) // 2
    inv = _merge_count(a, tmp, lo, mid) + _merge_count(a, tmp, mid, hi)
    i, j, k = lo, mid, lo
    while i < mid and j < hi:
        if a[i] <= a[j]:
            tmp[k] = a[i]
            i += 1
        else:
            tmp[k] = a[j]
            j += 1
            inv += mid - i
        k += 1
    while i < mid:
        tmp[k] = a[i]; i += 1; k += 1
    while j < hi:
        tmp[k] = a[j]; j += 1; k += 1
    a[lo:hi] = tmp[lo:hi]
    return inv


def count_inversions(arr):
    a = list(arr)
    tmp = [0] * len(a)
    return _merge_count(a, tmp, 0, len(a))


def main():
    print("[2, 4, 1, 3, 5] has {} inversions".format(count_inversions([2, 4, 1, 3, 5])))
    print("[5, 4, 3, 2, 1] has {} inversions".format(count_inversions([5, 4, 3, 2, 1])))


if __name__ == "__main__":
    main()
