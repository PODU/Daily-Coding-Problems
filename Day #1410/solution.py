# Day 1410: Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
# check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.

COLS, ROWS = 7, 6


class Connect4:
    def __init__(self):
        self.grid = [['.'] * COLS for _ in range(ROWS)]
        self.turn = 'R'

    def drop(self, col):
        if col < 0 or col >= COLS or self.grid[ROWS - 1][col] != '.':
            return 'X'
        r = 0
        while self.grid[r][col] != '.':
            r += 1
        self.grid[r][col] = self.turn
        who = self.turn
        self.turn = 'B' if self.turn == 'R' else 'R'
        return who if self.wins(r, col, who) else ' '

    def wins(self, r, c, p):
        for dr, dc in ((0, 1), (1, 0), (1, 1), (1, -1)):
            cnt = 1
            for s in (-1, 1):
                for k in range(1, 4):
                    nr, nc = r + dr * k * s, c + dc * k * s
                    if not (0 <= nr < ROWS and 0 <= nc < COLS) or self.grid[nr][nc] != p:
                        break
                    cnt += 1
            if cnt >= 4:
                return True
        return False

    def show(self):
        for r in range(ROWS - 1, -1, -1):
            print(' '.join(self.grid[r]))


if __name__ == "__main__":
    game = Connect4()
    winner = ' '
    for m in [0, 1, 0, 1, 0, 1, 0]:  # Red wins vertically in column 0
        res = game.drop(m)
        if res in ('R', 'B'):
            winner = res
            break
    game.show()
    print(("Red" if winner == 'R' else "Black") + " wins!" if winner != ' ' else "No winner yet.")
