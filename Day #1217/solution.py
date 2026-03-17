# Day 1217: Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.

def count_set_bits(n: int) -> int:
    total = 0
    p = 2
    while p <= 2 * n:
        full = ((n + 1) // p) * (p // 2)
        rem = max(0, (n + 1) % p - p // 2)
        total += full + rem
        p <<= 1
    return total


if __name__ == "__main__":
    print(count_set_bits(5))
