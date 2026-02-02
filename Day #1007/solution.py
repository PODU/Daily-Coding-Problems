# Day 1007: King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
# Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).

def in_board(r, c):
    return 0 <= r < 8 and 0 <= c < 8


def is_check(b):
    kr = kc = -1
    for r in range(8):
        for c in range(8):
            if b[r][c] == 'K':
                kr, kc = r, c

    for dr, dc in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        r, c = kr + dr, kc + dc
        while in_board(r, c):
            p = b[r][c]
            if p != '.':
                if p in ('R', 'Q'):
                    return True
                break
            r += dr; c += dc

    for dr, dc in [(1, 1), (1, -1), (-1, 1), (-1, -1)]:
        r, c = kr + dr, kc + dc
        while in_board(r, c):
            p = b[r][c]
            if p != '.':
                if p in ('B', 'Q'):
                    return True
                break
            r += dr; c += dc

    for dr, dc in [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]:
        r, c = kr + dr, kc + dc
        if in_board(r, c) and b[r][c] == 'N':
            return True

    for dr in (-1, 0, 1):
        for dc in (-1, 0, 1):
            if dr == 0 and dc == 0:
                continue
            r, c = kr + dr, kc + dc
            if in_board(r, c) and b[r][c] == 'K':
                return True

    for dc in (-1, 1):
        r, c = kr + 1, kc + dc
        if in_board(r, c) and b[r][c] == 'P':
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
    print("True" if is_check(board) else "False")  # True
