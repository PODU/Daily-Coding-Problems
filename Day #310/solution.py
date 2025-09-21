# Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
def count_bits(N):
    total = 0
    i = 0
    while (1 << i) <= N:
        blk = 1 << (i + 1)
        full = (N + 1) // blk * (1 << i)
        rem = max(0, (N + 1) % blk - (1 << i))
        total += full + rem
        i += 1
    return total


if __name__ == "__main__":
    print(count_bits(5))  # 7
