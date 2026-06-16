# Day 1678: Integer division without / * %. Subtract largest shifted multiple of
# divisor each round (doubling). Time O(log(quotient)), Space O(1).


def divide(a, b):
    q = 0
    while a >= b:
        temp, mult = b, 1
        while a >= (temp << 1):
            temp <<= 1
            mult <<= 1
        a -= temp
        q += mult
    return q


if __name__ == "__main__":
    print(divide(43, 8))  # 5
