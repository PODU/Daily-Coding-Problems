# Day 523: a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
# Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
def count_pairs(M, N):
    if M < N or (M - N) & 1:
        return 0
    c = (M - N) // 2
    if c & N:
        return 0
    ways = 1 << bin(N).count("1")
    if c == 0:
        ways -= 2
    return max(ways, 0)


if __name__ == "__main__":
    print(count_pairs(10, 4))  # 2 -> (3,7) and (7,3)
