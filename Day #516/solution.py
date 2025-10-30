# Day 516: Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
def sevenish(n):
    total, pow7 = 0, 1
    while n > 0:
        if n & 1:
            total += pow7
        pow7 *= 7
        n >>= 1
    return total


if __name__ == "__main__":
    # First few sevenish numbers: 1, 7, 8, 49, ...
    print(", ".join(str(sevenish(n)) for n in range(1, 5)))
