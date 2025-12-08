# Day 718: Gray code for n bits via formula g(i) = i ^ (i >> 1). Produces 2^n
# values where consecutive (and wrap-around) differ by one bit. Time O(2^n).

def gray_code(n):
    return [format(i ^ (i >> 1), '0{}b'.format(n)) for i in range(1 << n)]


if __name__ == "__main__":
    codes = gray_code(2)
    print("[" + ", ".join(codes) + "]")
