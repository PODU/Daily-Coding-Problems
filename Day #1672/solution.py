# Day 1672: Determine if black king is in check on an 8x8 board.
# Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).


def in_check(b):
    kr = kc = -1
    for r in range(8):
        for c in range(8):
            if b[r][c] == 'K':
                kr, kc = r, c

    for dr, dc in [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)]:
        r, c = kr + dr, kc + dc
        if 0 <= r < 8 and 0 <= c < 8 and b[r][c] == 'N':
            return True

    dirs = [(1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)]
    for d, (dr, dc) in enumerate(dirs):
        diag = d >= 4
        for step in range(1, 8):
            r, c = kr + dr*step, kc + dc*step
            if not (0 <= r < 8 and 0 <= c < 8):
                break
            p = b[r][c]
            if p == '.':
                continue
            if diag:
                if p in ('B', 'Q'):
                    return True
                if step == 1 and p == 'K':
                    return True
                if step == 1 and p == 'P' and dr == 1:
                    return True
            else:
                if p in ('R', 'Q'):
                    return True
                if step == 1 and p == 'K':
                    return True
            break
    return False


if __name__ == "__main__":
    board = ["...K....", "........", ".B......", "......P.",
             ".......R", "..N.....", "........", ".....Q.."]
    print("True" if in_check(board) else "False")  # True
