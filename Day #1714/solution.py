# Day 1714: Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
DIRS = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def solve(matrix, dictionary):
    n = len(matrix)
    taken = [[False] * n for _ in range(n)]
    words = list(dictionary)
    best = 0

    def find_placements(word):
        found = set()

        def dfs(r, c, idx, path):
            if r < 0 or r >= n or c < 0 or c >= n:
                return
            if taken[r][c] or (r, c) in path or matrix[r][c] != word[idx]:
                return
            path.add((r, c))
            if idx == len(word) - 1:
                found.add(frozenset(path))
            else:
                for dr, dc in DIRS:
                    dfs(r + dr, c + dc, idx + 1, path)
            path.remove((r, c))

        for i in range(n):
            for j in range(n):
                dfs(i, j, 0, set())
        return [set(f) for f in found]

    def search(used):
        nonlocal best
        best = max(best, len(used))
        for w in words:
            if w in used:
                continue
            for placement in find_placements(w):
                for (r, c) in placement:
                    taken[r][c] = True
                used.add(w)
                search(used)
                used.remove(w)
                for (r, c) in placement:
                    taken[r][c] = False

    search(set())
    return best


if __name__ == "__main__":
    dictionary = {'eat', 'rain', 'in', 'rat'}
    matrix = [['e', 'a', 'n'], ['t', 't', 'i'], ['a', 'r', 'a']]
    print(solve(matrix, dictionary))
