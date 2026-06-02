# Day 1603: Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
# For each bit i: full cycles contribute 2^i ones each, plus remainder.

def count_set_bits(n: int) -> int:
    total = 0
    i = 0
    while (1 << i) <= n:
        block = 1 << (i + 1)
        ones = (n + 1) // block * (1 << i)
        rem = (n + 1) % block - (1 << i)
        if rem > 0:
            ones += rem
        total += ones
        i += 1
    return total


if __name__ == "__main__":
    print(f"N=5  -> {count_set_bits(5)}")   # 7
    print(f"N=16 -> {count_set_bits(16)}")  # 33
