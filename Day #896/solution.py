# Day 896: Integer division without / * %: subtract largest shifted multiple of divisor.
# Bit-shifting. Time O((log n)^2), Space O(1).
def divide(dividend, divisor):
    quotient = 0
    while dividend >= divisor:
        temp, multiple = divisor, 1
        while dividend >= (temp << 1):
            temp <<= 1
            multiple <<= 1
        dividend -= temp
        quotient += multiple
    return quotient


if __name__ == "__main__":
    print(divide(43, 8))
    print(divide(100, 9))
