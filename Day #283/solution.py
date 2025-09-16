# Day 283: First N regular (5-smooth) numbers via 3-pointer merge of 2x,3x,5x.
# Time O(N), Space O(N).


def regular(n):
    h = [1] * n
    i2 = i3 = i5 = 0
    for i in range(1, n):
        n2, n3, n5 = h[i2] * 2, h[i3] * 3, h[i5] * 5
        nxt = min(n2, n3, n5)
        h[i] = nxt
        if nxt == n2:
            i2 += 1
        if nxt == n3:
            i3 += 1
        if nxt == n5:
            i5 += 1
    return h


if __name__ == "__main__":
    print(regular(10))  # [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
