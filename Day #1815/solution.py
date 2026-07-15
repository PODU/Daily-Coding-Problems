# Day 1815: Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
# Time: O(n * 2^n) to format. Space: O(2^n).


def gray_code(n):
    return [format(i ^ (i >> 1), "0{}b".format(n)) for i in range(1 << n)]


if __name__ == "__main__":
    print("[" + ", ".join(gray_code(2)) + "]")  # [00, 01, 11, 10]
