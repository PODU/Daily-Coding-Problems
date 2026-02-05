# Day 1022: Single number where all others appear 3 times.
# Approach: ones/twos bitmask accumulators. Time O(N), Space O(1).


def single_number(nums):
    ones = twos = 0
    for x in nums:
        ones = (ones ^ x) & ~twos
        twos = (twos ^ x) & ~ones
    return ones


if __name__ == "__main__":
    for t in ([6, 1, 3, 3, 3, 6, 6], [13, 19, 13, 13]):
        print(single_number(t))
