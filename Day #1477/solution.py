# Day 1477: First N regular numbers (only prime factors 2, 3, 5).
# DP with three pointers merging the sequences *2, *3, *5.
# Time O(N), Space O(N).

def regular_numbers(n):
    if n <= 0:
        return []
    h = [1] * n
    i2 = i3 = i5 = 0
    for k in range(1, n):
        nxt = min(h[i2] * 2, h[i3] * 3, h[i5] * 5)
        h[k] = nxt
        if nxt == h[i2] * 2:
            i2 += 1
        if nxt == h[i3] * 3:
            i3 += 1
        if nxt == h[i5] * 5:
            i5 += 1
    return h


if __name__ == "__main__":
    print(regular_numbers(10))  # [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
