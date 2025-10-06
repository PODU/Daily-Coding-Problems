# Day 380: Integer division without '/': repeated doubling/subtraction.
# Time: O(log n), Space: O(1).

def divide(a, b):
    neg = (a < 0) != (b < 0)
    x, y = abs(a), abs(b)
    q = 0
    while x >= y:
        temp, mult = y, 1
        while x >= (temp << 1):
            temp <<= 1
            mult <<= 1
        x -= temp
        q += mult
    return (-q if neg else q, x)


if __name__ == "__main__":
    print(divide(10, 3))
