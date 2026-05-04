# Day 1469: Chess check detection: locate black king, test pawn/knight attacks and ray-cast
# bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.


def in_check(b):
    kr = kc = -1
    for r in range(8):
        for c in range(8):
            if b[r][c] == 'K':
                kr, kc = r, c
    if kr < 0:
        return False

    # White pawns move up; a pawn at (kr+1, kc+-1) attacks the king.
    for dc in (-1, 1):
        pr, pc = kr + 1, kc + dc
        if 0 <= pr < 8 and 0 <= pc < 8 and b[pr][pc] == 'P':
            return True

    for dr, dc in ((1, 2), (1, -2), (-1, 2), (-1, -2),
                   (2, 1), (2, -1), (-2, 1), (-2, -1)):
        r, c = kr + dr, kc + dc
        if 0 <= r < 8 and 0 <= c < 8 and b[r][c] == 'N':
            return True

    for dr, dc in ((1, 1), (1, -1), (-1, 1), (-1, -1)):
        r, c = kr + dr, kc + dc
        while 0 <= r < 8 and 0 <= c < 8:
            p = b[r][c]
            if p != '.':
                if p in ('B', 'Q'):
                    return True
                break
            r += dr
            c += dc

    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        r, c = kr + dr, kc + dc
        while 0 <= r < 8 and 0 <= c < 8:
            p = b[r][c]
            if p != '.':
                if p in ('R', 'Q'):
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
    print("True" if in_check(board) else "False")
