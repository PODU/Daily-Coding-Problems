# Day 219: Connect 4 (7 cols x 6 rows).
# Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
# Win check O(1) per move; board O(W*H).
class Connect4:
    W, H = 7, 6

    def __init__(self):
        self.grid = [['.'] * self.W for _ in range(self.H)]
        self.height = [0] * self.W

    def drop(self, col: int, player: str) -> int:
        if col < 0 or col >= self.W or self.height[col] >= self.H:
            return -1
        r = self.height[col]
        self.height[col] += 1
        self.grid[r][col] = player
        return r

    def wins(self, r: int, c: int, p: str) -> bool:
        for dr, dc in ((0, 1), (1, 0), (1, 1), (1, -1)):
            cnt = 1
            for s in (-1, 1):
                rr, cc = r + dr * s, c + dc * s
                while 0 <= rr < self.H and 0 <= cc < self.W and self.grid[rr][cc] == p:
                    cnt += 1
                    rr += dr * s
                    cc += dc * s
            if cnt >= 4:
                return True
        return False


if __name__ == "__main__":
    g = Connect4()
    moves = [(0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R')]
    winner = None
    for col, player in moves:
        row = g.drop(col, player)
        if g.wins(row, col, player):
            winner = player
            break
    print(f"Player {winner} wins!")  # R wins vertically in column 0
