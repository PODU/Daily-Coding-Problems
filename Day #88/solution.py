# Day 88: Integer division using subtraction of shifted divisor (doubling).
# Time O(log^2 q), Space O(1).


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
    print(divide(10, 3))  # 3
    print(divide(27, 4))  # 6
