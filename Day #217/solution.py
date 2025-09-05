# Day 217: Smallest sparse number (no two adjacent set bits) >= N.
# Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
# Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
def next_sparse(n: int) -> int:
    if n <= 0:
        return 0
    bits = []
    x = n
    while x:
        bits.append(x & 1)
        x >>= 1
    bits.extend([0, 0])  # headroom for carries
    last_final = 0
    for i in range(1, len(bits) - 1):
        if bits[i] == 1 and bits[i - 1] == 1 and bits[i + 1] == 0:
            bits[i + 1] = 1
            for j in range(i, last_final - 1, -1):
                bits[j] = 0
            last_final = i + 1
    res = 0
    for i in range(len(bits) - 1, -1, -1):
        res = (res << 1) | bits[i]
    return res


if __name__ == "__main__":
    print(next_sparse(22))  # 22 = 10110 -> 32
    print(next_sparse(21))  # 21 = 10101 already sparse -> 21
