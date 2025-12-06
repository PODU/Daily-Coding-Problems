# Day 703: Probability a knight stays on an 8x8 board after k random moves.
# Approach: DP over board cells; each move spreads prob/8 to valid targets.
# Time O(k * N^2 * 8), Space O(N^2).

MOVES = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]


def knight_probability(n, k, r, c):
    dp = [[0.0] * n for _ in range(n)]
    dp[r][c] = 1.0
    for _ in range(k):
        nd = [[0.0] * n for _ in range(n)]
        for i in range(n):
            for j in range(n):
                if dp[i][j]:
                    for dr, dc in MOVES:
                        ni, nj = i + dr, j + dc
                        if 0 <= ni < n and 0 <= nj < n:
                            nd[ni][nj] += dp[i][j] / 8.0
        dp = nd
    return sum(sum(row) for row in dp)


if __name__ == "__main__":
    print(knight_probability(8, 2, 0, 0))  # 0.1875
