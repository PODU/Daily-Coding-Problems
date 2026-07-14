# Day 1810: 3-sum decision: does any triple sum to k?
# Sort, then for each i two-pointer scan. Time: O(n^2). Space: O(1).


def three_sum(a, k):
    a = sorted(a)
    n = len(a)
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = a[i] + a[lo] + a[hi]
            if s == k:
                return True
            elif s < k:
                lo += 1
            else:
                hi -= 1
    return False


if __name__ == "__main__":
    print(str(three_sum([20, 303, 3, 4, 25], 49)).lower())  # true
