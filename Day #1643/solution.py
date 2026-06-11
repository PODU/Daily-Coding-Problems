# Day 1643: Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
# the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).

def reverse_bits(n: int) -> int:
    result = 0
    for _ in range(32):
        result = ((result << 1) | (n & 1)) & 0xFFFFFFFF
        n >>= 1
    return result


def group_nibbles(n: int) -> str:
    bits = format(n & 0xFFFFFFFF, '032b')
    return ' '.join(bits[i:i + 4] for i in range(0, 32, 4))


if __name__ == "__main__":
    value = 0xF0F0F0F0
    print(group_nibbles(reverse_bits(value)))
