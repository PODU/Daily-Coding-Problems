# Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
# rays and bishop/queen diagonal rays (stopping at the first blocker), and test
# knight and pawn attacks. Time O(8x8) = O(1); space O(1).


def in_check(board):
    kr = kc = -1
    for r in range(8):
        for c in range(8):
            if board[r][c] == 'K':
                kr, kc = r, c
    if kr < 0:
        return False

    def at(r, c):
        if 0 <= r < 8 and 0 <= c < 8:
            return board[r][c]
        return None

    # Orthogonal rays: Rook or Queen
    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        r, c = kr + dr, kc + dc
        while at(r, c) == '.':
            r, c = r + dr, c + dc
        if at(r, c) in ('R', 'Q'):
            return True
    # Diagonal rays: Bishop or Queen
    for dr, dc in ((1, 1), (1, -1), (-1, 1), (-1, -1)):
        r, c = kr + dr, kc + dc
        while at(r, c) == '.':
            r, c = r + dr, c + dc
        if at(r, c) in ('B', 'Q'):
            return True
    # Knight
    for dr, dc in ((1, 2), (1, -2), (-1, 2), (-1, -2),
                   (2, 1), (2, -1), (-2, 1), (-2, -1)):
        if at(kr + dr, kc + dc) == 'N':
            return True
    # White pawn attacks diagonally above it (row-1), so a pawn at (kr+1, kc+-1)
    if at(kr + 1, kc - 1) == 'P' or at(kr + 1, kc + 1) == 'P':
        return True
    # Enemy king adjacency
    for dr in (-1, 0, 1):
        for dc in (-1, 0, 1):
            if (dr or dc) and at(kr + dr, kc + dc) == 'k':
                return True
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
    print("True" if in_check(board) else "False")
