# Day 462: Smallest sparse number (no adjacent set bits) >= N.
# Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).


def next_sparse(n):
    if n <= 0:
        return n
    bits = []
    t = n
    while t > 0:
        bits.append(t & 1)
        t >>= 1
    bits += [0, 0]  # padding for carries
    size = len(bits)
    last_final = 0
    for i in range(1, size - 1):
        if bits[i] == 1 and bits[i - 1] == 1 and bits[i + 1] == 0:
            bits[i + 1] = 1
            for j in range(i, last_final - 1, -1):
                bits[j] = 0
            last_final = i + 1
    ans = 0
    for i in range(size):
        if bits[i]:
            ans |= (1 << i)
    return ans


if __name__ == "__main__":
    print(next_sparse(22))  # 32
    print(next_sparse(21))  # 21
