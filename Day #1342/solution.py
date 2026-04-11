# Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
# Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
from typing import List


def max_words(dictionary: List[str], board: List[List[str]]) -> int:
    N, M = len(board), len(board[0])
    dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]

    def placements_for(word: str):
        masks = set()

        def dfs(idx: int, r: int, c: int, mask: int):
            mask |= 1 << (r * M + c)
            if idx == len(word) - 1:
                masks.add(mask)
                return
            for dr, dc in dirs:
                nr, nc = r + dr, c + dc
                if 0 <= nr < N and 0 <= nc < M and not (mask >> (nr * M + nc)) & 1 \
                        and board[nr][nc] == word[idx + 1]:
                    dfs(idx + 1, nr, nc, mask)

        for r in range(N):
            for c in range(M):
                if board[r][c] == word[0]:
                    dfs(0, r, c, 0)
        return list(masks)

    placements = [p for w in dictionary if (p := placements_for(w))]

    best = 0

    def backtrack(i: int, used: int, count: int):
        nonlocal best
        if count + (len(placements) - i) <= best:
            return
        if i == len(placements):
            best = max(best, count)
            return
        backtrack(i + 1, used, count)  # skip word
        for m in placements[i]:
            if not (m & used):
                backtrack(i + 1, used | m, count + 1)

    backtrack(0, 0, 0)
    return best


if __name__ == "__main__":
    dictionary = {"eat", "rain", "in", "rat"}
    matrix = [["e", "a", "n"],
              ["t", "t", "i"],
              ["a", "r", "a"]]
    print(max_words(list(dictionary), matrix))
