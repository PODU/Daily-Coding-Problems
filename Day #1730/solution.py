# Day 1730: Fast integer exponentiation (exponentiation by squaring).
# Square the base and halve the exponent each step. Time: O(log y). Space: O(1).


def fast_pow(x, y):
    if y < 0:  # x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
        inv = fast_pow(x, -y)
        return 0 if inv == 0 else 1 // inv
    result, base = 1, x
    while y > 0:
        if y & 1:
            result *= base
        base *= base
        y >>= 1
    return result


def main():
    print(fast_pow(2, 10))  # 1024


if __name__ == "__main__":
    main()
