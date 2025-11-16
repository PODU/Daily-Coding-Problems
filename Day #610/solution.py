# Day 610: Integer division of positive ints without / , * , or %.
# Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).


def divide(dividend, divisor):
    q = 0
    while dividend >= divisor:
        temp, mult = divisor, 1
        while dividend >= (temp << 1):
            temp <<= 1
            mult <<= 1
        dividend -= temp
        q += mult
    return q


def main():
    print(divide(10, 3))  # 3
    print(divide(43, 8))  # 5


if __name__ == '__main__':
    main()
