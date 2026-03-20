# Day 1239: Integer division without * / %. Subtract shifted divisor (doubling).
# Time O(log^2 q), Space O(1).


def divide(dividend, divisor):
    quotient = 0
    while dividend >= divisor:
        temp, multiple = divisor, 1
        while (temp << 1) <= dividend:
            temp <<= 1
            multiple <<= 1
        dividend -= temp
        quotient += multiple
    return quotient


if __name__ == "__main__":
    print(divide(43, 5))  # 8
    print(divide(100, 10))  # 10
