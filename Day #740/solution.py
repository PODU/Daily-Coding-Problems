# Day 740: First N regular (5-smooth / Hamming) numbers via 3-pointer dynamic programming.
# Each number is min of next multiples by 2, 3, 5.
# Time: O(N), Space: O(N).

def regular_numbers(n):
    h = [1] * n
    i2 = i3 = i5 = 0
    for i in range(1, n):
        n2, n3, n5 = h[i2] * 2, h[i3] * 3, h[i5] * 5
        nx = min(n2, n3, n5)
        h[i] = nx
        if nx == n2:
            i2 += 1
        if nx == n3:
            i3 += 1
        if nx == n5:
            i5 += 1
    return h


if __name__ == "__main__":
    print(*regular_numbers(10))  # 1 2 3 4 5 6 8 9 10 12
