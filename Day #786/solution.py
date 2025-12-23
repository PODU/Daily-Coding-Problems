# Day 786: Integer exponentiation by squaring. Time O(log y), Space O(1).
# Negative y handled via reciprocal; demo uses pow(2, 10).

def ipow(x, y):
    if y < 0:
        return 1.0 / ipow(x, -y)
    result = 1
    base = x
    e = y
    while e > 0:
        if e & 1:
            result *= base
        base *= base
        e >>= 1
    return result


if __name__ == "__main__":
    print(ipow(2, 10))
