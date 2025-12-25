# Day 801: nth sevenish number = sum of unique powers of 7.
# Bits of n select which powers of 7 to add (base-7 analog of binary).
# Time O(log n), Space O(1).


def sevenish(n):
    result, power = 0, 1
    while n:
        if n & 1:
            result += power
        power *= 7
        n >>= 1
    return result


if __name__ == "__main__":
    print(" ".join(str(sevenish(i)) for i in range(1, 6)))  # 1 7 8 49 50
