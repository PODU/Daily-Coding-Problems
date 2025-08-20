# Day 148: Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).

def gray_code(n):
    return [format(i ^ (i >> 1), '0{}b'.format(n)) for i in range(1 << n)]


if __name__ == "__main__":
    n = 2
    codes = gray_code(n)
    print("[" + ", ".join(codes) + "]")
