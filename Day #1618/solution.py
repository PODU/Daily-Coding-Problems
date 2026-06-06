# Day 1618: Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).

def sevenish(n):
    total, pow7 = 0, 1
    while n > 0:
        if n & 1:
            total += pow7
        pow7 *= 7
        n >>= 1
    return total


if __name__ == "__main__":
    print(" ".join(str(sevenish(n)) for n in range(1, 6)))
