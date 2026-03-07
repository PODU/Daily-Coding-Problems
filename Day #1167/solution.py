# Day 1167: Fast (binary) exponentiation by squaring on an integer result.
# Time: O(log y), Space: O(1).

def fast_pow(x, y):
    result = 1
    while y > 0:
        if y & 1:
            result *= x
        x *= x
        y >>= 1
    return result


if __name__ == "__main__":
    print(fast_pow(2, 10))
