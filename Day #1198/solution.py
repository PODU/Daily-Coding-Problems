# Day 1198: Recover coin denominations from change-ways array A (unbounded coin change).
# dp starts [1,0,...]; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.

def recover_coins(A):
    n = len(A)
    dp = [0] * n
    dp[0] = 1
    coins = []
    for i in range(1, n):
        if A[i] != dp[i]:
            coins.append(i)
            for v in range(i, n):
                dp[v] += dp[v - i]
    return coins


def format_list(xs):
    n = len(xs)
    parts = []
    for i, x in enumerate(xs):
        if i == n - 1 and n > 1:
            parts.append("and " + str(x))
        else:
            parts.append(str(x))
    return ", ".join(parts)


if __name__ == "__main__":
    A = [1, 0, 1, 1, 2]
    print(format_list(recover_coins(A)))  # 2, 3, and 4
