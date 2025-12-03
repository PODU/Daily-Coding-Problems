# Day 690: Longest strictly Increasing Subsequence length via patience sorting + binary search.
# Time O(N log N), Space O(N). Also reconstructs one valid LIS.
from bisect import bisect_left


def lis_length(nums):
    tails = []          # tails[k] = smallest tail of an increasing subseq of length k+1
    tails_idx = []      # index in nums of that tail
    prev = [-1] * len(nums)
    for i, x in enumerate(nums):
        pos = bisect_left(tails, x)
        if pos == len(tails):
            tails.append(x)
            tails_idx.append(i)
        else:
            tails[pos] = x
            tails_idx[pos] = i
        prev[i] = tails_idx[pos - 1] if pos > 0 else -1
    # reconstruct
    seq = []
    k = tails_idx[-1] if tails_idx else -1
    while k != -1:
        seq.append(nums[k])
        k = prev[k]
    seq.reverse()
    return len(tails), seq


def main():
    nums = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]
    length, seq = lis_length(nums)
    print(length)
    print(seq)


if __name__ == "__main__":
    main()
