# Day 61: Fast (binary) exponentiation: square-and-multiply. Time O(log y), Space O(1).

def fast_pow(x, y):
    if y < 0:
        return 1 / fast_pow(x, -y)
    result = 1
    while y > 0:
        if y & 1:
            result *= x
        x *= x
        y >>= 1
    return result


if __name__ == "__main__":
    print(fast_pow(2, 10))  # 1024
