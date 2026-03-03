# Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
# a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
# Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
def count_pairs(M, N):
    if M < N or (M - N) & 1:
        return 0
    a_and = (M - N) // 2
    if a_and & N:
        return 0
    cnt = 1 << bin(N).count("1")
    if a_and == 0:
        cnt -= 2
    return max(cnt, 0)


if __name__ == "__main__":
    print(count_pairs(10, 4))  # 2 -> (7,3) and (3,7)
