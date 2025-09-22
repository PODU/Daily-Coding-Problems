# Day 312: Tilings of a 2xN board with dominoes & L-trominoes.
# DP recurrence f(n) = 2*f(n-1) + f(n-3). O(N) time.
def tilings(n):
    if n == 0:
        return 1
    if n == 1:
        return 1
    if n == 2:
        return 2
    f = [0] * (n + 1)
    f[0], f[1], f[2] = 1, 1, 2
    for i in range(3, n + 1):
        f[i] = 2 * f[i - 1] + f[i - 3]
    return f[n]


if __name__ == "__main__":
    print(tilings(4))  # 11
