# Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.


def ipow(x, y):
    if y < 0:
        return 1.0 / ipow(x, -y)
    result = 1
    base = x
    while y > 0:
        if y & 1:
            result *= base
        base *= base
        y >>= 1
    return result


if __name__ == "__main__":
    print(ipow(2, 10))  # 1024
    print(ipow(3, 5))   # 243
    print(ipow(2, -2))  # 0.25
