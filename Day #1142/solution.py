# Day 1142: nth sevenish number = sum of distinct powers of 7.
# Bits of n in binary select powers of 7 (bijection). Time O(log n), Space O(1).
def sevenish(n):
    result, power = 0, 1
    while n > 0:
        if n & 1:
            result += power
        power *= 7
        n >>= 1
    return result


if __name__ == "__main__":
    print(*[sevenish(i) for i in range(1, 6)])  # 1 7 8 49 50
