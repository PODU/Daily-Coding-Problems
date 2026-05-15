# Day 1520: Single number among triples: bitwise ones/twos accumulators isolate the unique value.
# Time: O(n). Space: O(1).


def single_number(nums):
    ones = twos = 0
    for x in nums:
        # mask to 32-bit to handle negatives correctly in Python's arbitrary ints
        x &= 0xFFFFFFFF
        ones = (ones ^ x) & ~twos & 0xFFFFFFFF
        twos = (twos ^ x) & ~ones & 0xFFFFFFFF
    # convert back from 32-bit unsigned to signed
    return ones - (1 << 32) if ones >= (1 << 31) else ones


def main():
    a = [6, 1, 3, 3, 3, 6, 6]
    b = [13, 19, 13, 13]
    print(single_number(a))  # 1
    print(single_number(b))  # 19


if __name__ == "__main__":
    main()
