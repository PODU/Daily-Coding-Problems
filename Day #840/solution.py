# Day 840: Print a string in zigzag form across k lines.
# Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
# Build k rows of spaces, place each char, print with trailing spaces trimmed.  Time O(N*k), Space O(N*k).


def zigzag(s, k):
    if k <= 0:
        return ""
    if k == 1:
        return s
    n = len(s)
    rows = [[" "] * n for _ in range(k)]
    row, step = 0, 1
    for i, ch in enumerate(s):
        rows[row][i] = ch
        if row == 0:
            step = 1
        elif row == k - 1:
            step = -1
        row += step
    return "\n".join("".join(r).rstrip() for r in rows)


if __name__ == "__main__":
    print(zigzag("thisisazigzag", 4))
