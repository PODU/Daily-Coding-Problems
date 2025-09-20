# Day 304: Probability knight stays on board after k random moves. DP over board.
# Time O(k*N^2*8), space O(N^2).
def knight_prob(N, k, sr, sc):
    dp = [[0.0] * N for _ in range(N)]
    dp[sr][sc] = 1.0
    moves = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]
    for _ in range(k):
        nd = [[0.0] * N for _ in range(N)]
        for r in range(N):
            for c in range(N):
                if dp[r][c]:
                    for dr, dc in moves:
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < N and 0 <= nc < N:
                            nd[nr][nc] += dp[r][c] / 8.0
        dp = nd
    return sum(sum(row) for row in dp)


if __name__ == "__main__":
    print(knight_prob(8, 1, 0, 0))  # 0.25
