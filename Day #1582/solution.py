# Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
# drop() places a disc; win() scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).

COLS, ROWS = 7, 6


class Connect4:
    def __init__(self):
        self.g = [["."] * COLS for _ in range(ROWS)]

    def drop(self, col, p):
        for r in range(ROWS - 1, -1, -1):
            if self.g[r][col] == ".":
                self.g[r][col] = p
                return r
        return -1

    def win(self, r, c):
        p = self.g[r][c]
        for dr, dc in ((0, 1), (1, 0), (1, 1), (1, -1)):
            cnt = 1
            for s in (-1, 1):
                rr, cc = r + s * dr, c + s * dc
                while 0 <= rr < ROWS and 0 <= cc < COLS and self.g[rr][cc] == p:
                    cnt += 1
                    rr += s * dr
                    cc += s * dc
            if cnt >= 4:
                return True
        return False

    def show(self):
        for row in self.g:
            print(" ".join(row))


if __name__ == "__main__":
    game = Connect4()
    moves = [(0, "R"), (1, "B"), (0, "R"), (1, "B"), (0, "R"), (1, "B"), (0, "R")]
    winner = None
    for col, p in moves:
        r = game.drop(col, p)
        if game.win(r, col):
            winner = p
            break
    game.show()
    print("Winner:", winner if winner else "none")  # R
