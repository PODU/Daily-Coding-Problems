# Day 907: Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.


def gray_code(n):
    return [format(i ^ (i >> 1), "0{}b".format(n)) for i in range(1 << n)]


if __name__ == "__main__":
    n = 2
    codes = gray_code(n)
    print("[" + ", ".join(codes) + "]")
