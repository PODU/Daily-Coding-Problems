# Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
# into the result. Time O(32), Space O(1).


def reverse_bits(n):
    res = 0
    for _ in range(32):
        res = (res << 1) | (n & 1)
        n >>= 1
    return res


def to_groups(n):
    bits = format(n & 0xFFFFFFFF, "032b")
    return " ".join(bits[i:i + 4] for i in range(0, 32, 4))


if __name__ == "__main__":
    n = 0xF0F0F0F0  # 1111 0000 ... repeated
    print(to_groups(reverse_bits(n)))
