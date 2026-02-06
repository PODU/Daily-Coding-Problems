# Day 1027: Longest consecutive elements sequence.
# Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).


def longest_consecutive(nums):
    s = set(nums)
    best = 0
    for x in s:
        if x - 1 not in s:
            length, cur = 1, x
            while cur + 1 in s:
                cur += 1
                length += 1
            best = max(best, length)
    return best


if __name__ == "__main__":
    print(longest_consecutive([100, 4, 200, 1, 3, 2]))
