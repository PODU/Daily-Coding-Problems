# Day 1390: Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.

def max_coins(m):
    R, C = len(m), len(m[0])
    dp = [[0] * C for _ in range(R)]
    for i in range(R):
        for j in range(C):
            best = 0
            if i > 0:
                best = max(best, dp[i-1][j])
            if j > 0:
                best = max(best, dp[i][j-1])
            dp[i][j] = m[i][j] + (0 if i == 0 and j == 0 else best)
    return dp[R-1][C-1]


def main():
    m = [
        [0, 3, 1, 1],
        [2, 0, 0, 4],
        [1, 5, 3, 1],
    ]
    print(max_coins(m))  # 12


if __name__ == "__main__":
    main()
