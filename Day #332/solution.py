# Day 332: Count ordered (a,b), a+b=M, a^b=N. c=(M-N)/2; valid if c&N==0.
# Count=2^popcount(N), minus 2 if M==N. Time O(1), Space O(1).
def count_pairs(M, N):
    diff = M - N
    if diff < 0 or (diff & 1):
        return 0
    c = diff // 2
    if c & N:
        return 0
    count = 1 << bin(N).count("1")
    if M == N:
        count -= 2
    return max(count, 0)


if __name__ == "__main__":
    print(count_pairs(10, 4))
