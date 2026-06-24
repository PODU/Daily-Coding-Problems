# Day 1706: Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
def longest_consecutive(nums):
    s = set(nums)
    best = 0
    for n in s:
        if n - 1 not in s:
            cur, length = n, 1
            while cur + 1 in s:
                cur += 1
                length += 1
            best = max(best, length)
    return best


if __name__ == "__main__":
    print(longest_consecutive([100, 4, 200, 1, 3, 2]))
