# Day 221: nth "sevenish" number (sum of distinct powers of 7).
# Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
def sevenish(n: int) -> int:
    result, power = 0, 1  # 7^0
    while n > 0:
        if n & 1:
            result += power
        power *= 7
        n >>= 1
    return result


if __name__ == "__main__":
    print(" ".join(str(sevenish(i)) for i in range(1, 6)))  # 1 7 8 49 50
    print(sevenish(4))  # 49
