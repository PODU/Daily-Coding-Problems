# Day 1092: First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
def regular(n):
    h = [1] * n
    i2 = i3 = i5 = 0
    for i in range(1, n):
        n2, n3, n5 = h[i2] * 2, h[i3] * 3, h[i5] * 5
        m = min(n2, n3, n5)
        h[i] = m
        if m == n2:
            i2 += 1
        if m == n3:
            i3 += 1
        if m == n5:
            i5 += 1
    return h


if __name__ == "__main__":
    print(regular(10))
