# Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
# with next-larger suffix element, reverse suffix. O(n) time, O(1) extra space.
def next_permutation(a):
    n = len(a)
    i = n - 2
    while i >= 0 and a[i] >= a[i + 1]:
        i -= 1
    if i >= 0:
        j = n - 1
        while a[j] <= a[i]:
            j -= 1
        a[i], a[j] = a[j], a[i]
    # reverse the suffix a[i+1:]
    lo, hi = i + 1, n - 1
    while lo < hi:
        a[lo], a[hi] = a[hi], a[lo]
        lo, hi = lo + 1, hi - 1
    return a


if __name__ == "__main__":
    for arr in ([1, 2, 3], [1, 3, 2], [3, 2, 1]):
        print(next_permutation(arr[:]))
    # [1, 3, 2]
    # [2, 1, 3]
    # [1, 2, 3]
