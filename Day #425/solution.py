# Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
# with blocking. O(1) board scan from the king's square.
def is_check(board):
    n = 8
    kr = kc = -1
    for i in range(n):
        for j in range(n):
            if board[i][j] == 'K':
                kr, kc = i, j

    def inb(r, c):
        return 0 <= r < n and 0 <= c < n

    # Knight L-shapes
    for dr, dc in [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)]:
        r, c = kr + dr, kc + dc
        if inb(r, c) and board[r][c] == 'N':
            return True

    # White pawn attacks toward smaller row -> pawn sits at (kr+1, kc+-1)
    for dc in (-1, 1):
        r, c = kr + 1, kc + dc
        if inb(r, c) and board[r][c] == 'P':
            return True

    # Rook / Queen orthogonal rays
    for dr, dc in [(-1,0),(1,0),(0,-1),(0,1)]:
        r, c = kr + dr, kc + dc
        while inb(r, c):
            p = board[r][c]
            if p != '.':
                if p in ('R', 'Q'):
                    return True
                break
            r += dr
            c += dc

    # Bishop / Queen diagonal rays
    for dr, dc in [(-1,-1),(-1,1),(1,-1),(1,1)]:
        r, c = kr + dr, kc + dc
        while inb(r, c):
            p = board[r][c]
            if p != '.':
                if p in ('B', 'Q'):
                    return True
                break
            r += dr
            c += dc

    return False


if __name__ == "__main__":
    board = [
        "...K....",
        "........",
        ".B......",
        "......P.",
        ".......R",
        "..N.....",
        "........",
        ".....Q..",
    ]
    print("True" if is_check(board) else "False")
