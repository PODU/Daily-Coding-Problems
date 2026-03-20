# Day 1240: Reflected binary Gray code: g(i) = i ^ (i >> 1).
# Time O(2^n), Space O(2^n).


def gray_code(n):
    return [format(i ^ (i >> 1), '0{}b'.format(n)) for i in range(1 << n)]


if __name__ == "__main__":
    print("[" + ", ".join(gray_code(2)) + "]")
