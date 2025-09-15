# Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
# Time O(N^2), Space O(N) for squares.


def has_triplet(arr):
    a = sorted(x * x for x in arr)
    n = len(a)
    for c in range(n - 1, 1, -1):
        lo, hi = 0, c - 1
        while lo < hi:
            s = a[lo] + a[hi]
            if s == a[c]:
                return True
            elif s < a[c]:
                lo += 1
            else:
                hi -= 1
    return False


if __name__ == "__main__":
    print(has_triplet([3, 1, 4, 6, 5]))    # True (3,4,5)
    print(has_triplet([10, 4, 6, 12, 5]))  # False
