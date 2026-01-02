# Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
# Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1).

def count_pairs(M, N):
    d = M - N
    if d < 0 or d & 1:
        return 0
    c = d // 2                      # c = a & b
    if c & N:                       # AND and XOR bits cannot overlap
        return 0
    res = 1 << bin(N).count('1')
    if c == 0:
        res -= 2 if N != 0 else 1
    return max(res, 0)


if __name__ == "__main__":
    print(count_pairs(4, 2))        # 2
