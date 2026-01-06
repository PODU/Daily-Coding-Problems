# Day 861: Probability a knight stays on an 8x8 board after k random moves.
# Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
# Time: O(k * N^2), Space: O(N^2).
MOVES = [(-2, -1), (-2, 1), (-1, -2), (-1, 2),
         (1, -2), (1, 2), (2, -1), (2, 1)]


def knight_probability(n, k, sr, sc):
    dp = [[0.0] * n for _ in range(n)]
    dp[sr][sc] = 1.0
    for _ in range(k):
        nx = [[0.0] * n for _ in range(n)]
        for r in range(n):
            for c in range(n):
                if dp[r][c]:
                    for dr, dc in MOVES:
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < n and 0 <= nc < n:
                            nx[nr][nc] += dp[r][c] / 8.0
        dp = nx
    return sum(v for row in dp for v in row)


if __name__ == "__main__":
    print(knight_probability(8, 1, 0, 0))  # 0.25
    print(knight_probability(8, 2, 0, 0))
