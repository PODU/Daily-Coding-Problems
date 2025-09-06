# Day 225: Josephus problem - position (1-indexed) of last survivor.
# Approach: general O(N) recurrence J(i)=(J(i-1)+k)%i. Bonus: k==2 closed form O(log N): 2*(n-2^floor(log2 n))+1.
def josephus(n: int, k: int) -> int:
    if k == 2:
        p = 1
        while p * 2 <= n:
            p *= 2  # highest power of 2 <= n
        return 2 * (n - p) + 1
    res = 0  # 0-indexed
    for i in range(2, n + 1):
        res = (res + k) % i
    return res + 1


if __name__ == "__main__":
    print(josephus(5, 2))  # 3
