# Day 1809: Knight-on-board probability after k random moves: DP over board states.
# dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
# Time: O(k*64*8). Space: O(64).

MOVES = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)]


def knight_probability(n, k, r0, c0):
    dp = [[0.0] * n for _ in range(n)]
    dp[r0][c0] = 1.0
    for _ in range(k):
        nxt = [[0.0] * n for _ in range(n)]
        for r in range(n):
            for c in range(n):
                if dp[r][c] == 0:
                    continue
                for dr, dc in MOVES:
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < n and 0 <= nc < n:
                        nxt[nr][nc] += dp[r][c] / 8.0
        dp = nxt
    return sum(v for row in dp for v in row)


if __name__ == "__main__":
    # corner (0,0), k=1 -> 2/8 = 0.25
    print(knight_probability(8, 1, 0, 0))  # 0.25
