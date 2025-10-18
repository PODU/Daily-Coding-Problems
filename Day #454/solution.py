# Day 454: Longest Increasing Subsequence length.
# Patience sorting with binary search. Time O(n log n), Space O(n).
from bisect import bisect_left


def lis(a):
    tails = []
    for x in a:
        i = bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    print(lis([10, 9, 2, 5, 3, 7, 101, 18]))  # 4
