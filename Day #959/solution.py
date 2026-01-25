# Day 959: total number of set bits over all integers in [1, N].
# Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).

def count_set_bits(n):
    total = 0
    i = 0
    while (1 << i) <= n:
        cycle = 1 << (i + 1)
        half = cycle >> 1
        total += (n + 1) // cycle * half
        rem = (n + 1) % cycle
        total += max(0, rem - half)
        i += 1
    return total


if __name__ == "__main__":
    print(count_set_bits(5))  # 7
