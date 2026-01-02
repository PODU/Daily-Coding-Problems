# Day 839: Max number of dictionary words packed on an NxN board.
# For each word collect all valid adjacent-path placements (DFS), then backtrack
# over words choosing a disjoint set to maximize the count.
# Time O(exponential worst-case) on small boards; placement search is bounded by board size.


def find_placements(matrix, word):
    """Return list of placements; each placement is a frozenset of (r,c) tiles."""
    n = len(matrix)
    placements = set()

    def dfs(r, c, idx, used):
        if matrix[r][c] != word[idx]:
            return
        used = used + [(r, c)]
        if idx == len(word) - 1:
            placements.add(frozenset(used))
            return
        for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < n and 0 <= nc < len(matrix[0]) and (nr, nc) not in used:
                dfs(nr, nc, idx + 1, used)

    for i in range(n):
        for j in range(len(matrix[0])):
            dfs(i, j, 0, [])
    return list(placements)


def max_words(matrix, dictionary):
    words = [w for w in dictionary if find_placements(matrix, w)]
    word_placements = [find_placements(matrix, w) for w in words]
    best = 0

    def backtrack(i, occupied, count):
        nonlocal best
        best = max(best, count)
        if i == len(words):
            return
        # Skip this word
        backtrack(i + 1, occupied, count)
        # Try each placement of this word
        for tiles in word_placements[i]:
            if occupied.isdisjoint(tiles):
                backtrack(i + 1, occupied | tiles, count + 1)

    backtrack(0, frozenset(), 0)
    return best


if __name__ == "__main__":
    dictionary = {"eat", "rain", "in", "rat"}
    matrix = [
        ["e", "a", "n"],
        ["t", "t", "i"],
        ["a", "r", "a"],
    ]
    print(max_words(matrix, dictionary))
