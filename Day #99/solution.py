# Day 99: Longest consecutive sequence. Put all in a set; start a count only at
# sequence beginnings (n-1 absent) and walk up. O(n) time, O(n) space.
def longest_consecutive(nums):
    s = set(nums)
    best = 0
    for n in s:
        if n - 1 not in s:  # n is a sequence start
            length = 1
            while n + length in s:
                length += 1
            best = max(best, length)
    return best


if __name__ == "__main__":
    print(longest_consecutive([100, 4, 200, 1, 3, 2]))  # 4
