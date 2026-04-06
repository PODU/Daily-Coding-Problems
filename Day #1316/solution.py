# Day 1316: Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
# top of each adjacent-ones run, carry up and clear below. Time O(log N).

def next_sparse(x):
    if x == 0:
        return 0
    b = []
    t = x
    while t:
        b.append(t & 1)
        t >>= 1
    b += [0, 0]  # padding for carries
    n = len(b)
    for i in range(1, n - 1):
        if b[i] == 1 and b[i - 1] == 1 and b[i + 1] == 0:
            b[i + 1] = 1
            for j in range(i + 1):
                b[j] = 0
    return sum(bit << i for i, bit in enumerate(b))


if __name__ == "__main__":
    print(next_sparse(22))  # 32
