# Day 341: Max non-overlapping dictionary words on a board.
# (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
# placements choosing pairwise-disjoint sets to maximize count.

def solve(board, dictionary):
    R, C = len(board), len(board[0])

    def dfs(w, idx, r, c, used, masks):
        if board[r][c] != w[idx]:
            return
        cell = r * C + c
        if used & (1 << cell):
            return
        used |= (1 << cell)
        if idx == len(w) - 1:
            masks.add(used)
            return
        for dr, dc in ((-1,0),(1,0),(0,-1),(0,1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < R and 0 <= nc < C:
                dfs(w, idx + 1, nr, nc, used, masks)

    placements = []  # (word_index, mask)
    for wi, w in enumerate(dictionary):
        masks = set()
        for r in range(R):
            for c in range(C):
                dfs(w, 0, r, c, 0, masks)
        for m in masks:
            placements.append((wi, m))

    best = 0

    def backtrack(start, occupied, used_words, count):
        nonlocal best
        best = max(best, count)
        for i in range(start, len(placements)):
            w, m = placements[i]
            if used_words & (1 << w):
                continue
            if occupied & m:
                continue
            backtrack(i + 1, occupied | m, used_words | (1 << w), count + 1)

    backtrack(0, 0, 0, 0)
    return best

def main():
    dictionary = ['eat', 'rain', 'in', 'rat']
    board = [['e','a','n'],['t','t','i'],['a','r','a']]
    print(solve(board, dictionary))

if __name__ == "__main__":
    main()
