# Day 1177: Find the element appearing once while all others appear 3 times.
# Track bits seen once (ones) and twice (twos); a third sighting clears both.
# Time O(N), Space O(1).


def single_number(a):
    ones = twos = 0
    for x in a:
        ones = (ones ^ x) & ~twos
        twos = (twos ^ x) & ~ones
    # Normalize for Python's unbounded ints (mask to 32-bit, handle sign).
    ones &= 0xFFFFFFFF
    return ones - (1 << 32) if ones >= (1 << 31) else ones


if __name__ == "__main__":
    print(single_number([6, 1, 3, 3, 3, 6, 6]))  # 1
    print(single_number([13, 19, 13, 13]))        # 19
