# Day 594: Max number of non-overlapping dictionary words packable on a letter board.
# Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
# then backtracking max set-packing to pick the most pairwise-disjoint placements.
# Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.


def max_words(board, dictionary):
    R, C = len(board), len(board[0])

    def find(word, idx, r, c, mask, out):
        if not (0 <= r < R and 0 <= c < C):
            return
        bit = 1 << (r * C + c)
        if mask & bit or board[r][c] != word[idx]:
            return
        mask |= bit
        if idx == len(word) - 1:
            out.add(mask)
            return
        for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            find(word, idx + 1, r + dr, c + dc, mask, out)

    placements = []
    for w in dictionary:
        masks = set()
        for r in range(R):
            for c in range(C):
                find(w, 0, r, c, 0, masks)
        placements.extend(masks)

    best = 0

    def pack(i, used, cnt):
        nonlocal best
        best = max(best, cnt)
        for j in range(i, len(placements)):
            if not (placements[j] & used):
                pack(j + 1, used | placements[j], cnt + 1)

    pack(0, 0, 0)
    return best


if __name__ == "__main__":
    board = [["e", "a", "n"], ["t", "t", "i"], ["a", "r", "a"]]
    dictionary = {"eat", "rain", "in", "rat"}
    print(max_words(board, dictionary))  # 3
