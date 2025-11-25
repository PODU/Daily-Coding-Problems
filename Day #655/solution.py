# Day 655: Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, shift input right.
# Time O(32)=O(1), space O(1). Mask with 0xFFFFFFFF for unsigned 32-bit arithmetic.

def reverse_bits(x):
    x &= 0xFFFFFFFF
    result = 0
    for _ in range(32):
        result = (result << 1) | (x & 1)
        x >>= 1
    return result & 0xFFFFFFFF


def main():
    out = reverse_bits(0xF0F0F0F0)
    bits = format(out, '032b')
    print(' '.join(bits[i:i + 4] for i in range(0, 32, 4)))


if __name__ == '__main__':
    main()
