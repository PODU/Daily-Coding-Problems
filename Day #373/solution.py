# Day 373: Longest run of consecutive integers formable from the list.
# Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.


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
    print(longest_consecutive([5, 2, 99, 3, 4, 1, 100]))  # 5
