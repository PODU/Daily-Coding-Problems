# Day 733: Connect 4 on a 7x6 grid.
# Approach: 2D board + per-column heights; after each drop scan 4 directions from the
# placed disc for 4-in-a-row. Time: O(1) per move, Space: O(rows*cols).

class Connect4:
    COLS, ROWS = 7, 6

    def __init__(self):
        self.board = [['.'] * self.COLS for _ in range(self.ROWS)]
        self.height = [0] * self.COLS

    def drop(self, col, color):
        r = self.height[col]
        self.height[col] += 1
        self.board[r][col] = color
        for dr, dc in [(0, 1), (1, 0), (1, 1), (1, -1)]:
            cnt = 1
            for s in (-1, 1):
                for k in range(1, 4):
                    nr, nc = r + dr * k * s, col + dc * k * s
                    if 0 <= nr < self.ROWS and 0 <= nc < self.COLS and self.board[nr][nc] == color:
                        cnt += 1
                    else:
                        break
            if cnt >= 4:
                return True
        return False

    def show(self):
        for r in range(self.ROWS - 1, -1, -1):
            print(' '.join(self.board[r]))


if __name__ == "__main__":
    game = Connect4()
    moves = [0, 0, 1, 1, 2, 2, 3]
    color, won = 'R', False
    for m in moves:
        won = game.drop(m, color)
        if won:
            break
        color = 'B' if color == 'R' else 'R'
    game.show()
    print("Red wins!" if won and color == 'R' else ("Black wins!" if won else "No winner yet"))
