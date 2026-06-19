# Day 1690: Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
# cell following trie edges with a visited set. O(cells * 8^L) worst, pruned by trie.

def solve(grid, words):
    R, C = len(grid), len(grid[0])
    trie = {}
    for w in words:
        node = trie
        for ch in w:
            node = node.setdefault(ch, {})
        node['#'] = w  # word end marker

    found = set()

    def dfs(r, c, node, visited):
        ch = grid[r][c]
        if ch not in node:
            return
        nxt = node[ch]
        if '#' in nxt:
            found.add(nxt['#'])
        visited.add((r, c))
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if dr == 0 and dc == 0:
                    continue
                nr, nc = r + dr, c + dc
                if 0 <= nr < R and 0 <= nc < C and (nr, nc) not in visited:
                    dfs(nr, nc, nxt, visited)
        visited.discard((r, c))

    for r in range(R):
        for c in range(C):
            dfs(r, c, trie, set())

    return sorted(found)

if __name__ == "__main__":
    grid = [['o', 'a', 'a', 'n'],
            ['e', 't', 'a', 'e'],
            ['i', 'h', 'k', 'r'],
            ['i', 'f', 'l', 'v']]
    dictionary = ["oath", "pea", "eat", "rain"]
    for w in solve(grid, dictionary):
        print(w)
