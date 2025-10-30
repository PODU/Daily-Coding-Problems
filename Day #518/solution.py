# Day 518: 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
def three_sum(a, k):
    a = sorted(a)
    n = len(a)
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = a[i] + a[lo] + a[hi]
            if s == k:
                return True
            if s < k:
                lo += 1
            else:
                hi -= 1
    return False


if __name__ == "__main__":
    print(str(three_sum([20, 303, 3, 4, 25], 49)).lower())
