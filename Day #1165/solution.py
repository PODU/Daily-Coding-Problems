# Day 1165: Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
# Time: O(k*64*8), Space: O(64).

def knight_probability(k, start_r, start_c):
    moves = [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)]
    prob = [[1.0] * 8 for _ in range(8)]
    for _ in range(k):
        nxt = [[0.0] * 8 for _ in range(8)]
        for r in range(8):
            for c in range(8):
                s = 0.0
                for dr, dc in moves:
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < 8 and 0 <= nc < 8:
                        s += prob[nr][nc]
                nxt[r][c] = s / 8.0
        prob = nxt
    return prob[start_r][start_c]


if __name__ == "__main__":
    ans = knight_probability(1, 0, 0)
    print(f"{ans:g}")
