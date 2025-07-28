# Day 44: Count Inversions via modified merge sort: count cross-pairs while merging.
# Time O(n log n), Space O(n).

def count_inversions(arr):
    a = list(arr)
    tmp = [0] * len(a)

    def rec(lo, hi):
        if hi - lo <= 1:
            return 0
        mid = (lo + hi) // 2
        inv = rec(lo, mid) + rec(mid, hi)
        i, j, k = lo, mid, lo
        while i < mid and j < hi:
            if a[i] <= a[j]:
                tmp[k] = a[i]; i += 1
            else:
                tmp[k] = a[j]; j += 1
                inv += mid - i
            k += 1
        while i < mid:
            tmp[k] = a[i]; i += 1; k += 1
        while j < hi:
            tmp[k] = a[j]; j += 1; k += 1
        a[lo:hi] = tmp[lo:hi]
        return inv

    return rec(0, len(a))


if __name__ == "__main__":
    print(count_inversions([2, 4, 1, 3, 5]))
    print(count_inversions([5, 4, 3, 2, 1]))
