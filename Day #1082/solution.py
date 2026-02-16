# Day 1082: Connect 4 engine with O(1)-per-move win detection (scan 4 directions from last disc).
# Time per move O(1), Space O(R*C).
class Connect4:
    R, C = 6, 7

    def __init__(self):
        self.g = [['.'] * self.C for _ in range(self.R)]
        self.cur = 'R'
        self.over = False
        self.winner = '.'

    def drop(self, col):
        if col < 0 or col >= self.C or self.over:
            return -1
        for r in range(self.R - 1, -1, -1):
            if self.g[r][col] == '.':
                self.g[r][col] = self.cur
                return r
        return -1

    def won(self, r, c):
        p = self.g[r][c]
        for dr, dc in ((0, 1), (1, 0), (1, 1), (1, -1)):
            cnt = 1
            for s in (-1, 1):
                nr, nc = r + dr * s, c + dc * s
                while 0 <= nr < self.R and 0 <= nc < self.C and self.g[nr][nc] == p:
                    cnt += 1
                    nr += dr * s
                    nc += dc * s
            if cnt >= 4:
                return True
        return False

    def full(self):
        return all(self.g[0][c] != '.' for c in range(self.C))

    def play(self, col):
        r = self.drop(col)
        if r < 0:
            return False
        if self.won(r, col):
            self.over = True
            self.winner = self.cur
        elif self.full():
            self.over = True
        else:
            self.cur = 'B' if self.cur == 'R' else 'R'
        return True

    def show(self):
        for row in self.g:
            print(' '.join(row))


if __name__ == "__main__":
    game = Connect4()
    for m in [0, 1, 0, 1, 0, 1, 0]:  # R wins vertically in column 0
        game.play(m)
    game.show()
    print(f"Player {game.winner} wins!" if game.winner != '.' else "Draw")
