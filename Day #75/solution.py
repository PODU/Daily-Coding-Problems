# Day 75: Longest strictly increasing subsequence via patience sorting (tails array + bisect_left).
# Time O(n log n), Space O(n).
from bisect import bisect_left


def length_of_lis(a):
    tails = []
    for x in a:
        i = bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    a = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]
    print(length_of_lis(a))
