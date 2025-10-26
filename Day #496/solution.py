# Day 496: Total set bits across 1..N.
# For each bit position, count how many numbers in [0,N] have it set using the
# periodic pattern. O(log N) time, O(1) space.


def count_set_bits(n):
    total = 0
    bit = 1
    while bit <= n:
        full = n + 1          # count of integers in [0, n]
        cycle = bit << 1      # period for this bit
        total += (full // cycle) * bit
        rem = full % cycle
        total += max(0, rem - bit)
        bit <<= 1
    return total


if __name__ == "__main__":
    print(count_set_bits(5))   # 7  (1+1+2+1+2)
    print(count_set_bits(16))  # 33
