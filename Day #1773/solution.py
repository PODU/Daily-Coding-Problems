# Day 1773: Count ordered positive pairs (a,b) with a+b=M, a^b=N.
# Use a+b=(a^b)+2*(a&b): s=(M-N)//2 must satisfy (s&N)==0; answer=2^popcount(N)
# minus the all-or-nothing assignments when s==0. O(1) time, O(1) space.


def count_pairs(M, N):
    if M - N < 0 or (M - N) & 1:
        return 0
    s = (M - N) // 2            # s == a & b
    if s & N:                   # carry bits must be disjoint from xor bits
        return 0
    if N == 0:
        return 1 if (M > 0 and M % 2 == 0) else 0  # only (M/2, M/2)
    ways = 1 << bin(N).count("1")
    if s == 0:
        ways -= 2               # drop a=0 and b=0 to keep both positive
    return ways


if __name__ == "__main__":
    print(count_pairs(6, 4))  # (1,5),(5,1) -> 2
