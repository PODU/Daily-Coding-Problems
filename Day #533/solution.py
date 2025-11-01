# Day 533: Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
# (each cell used once per path), collect words present in the trie.
# Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).


def solve(board, dictionary):
    trie = {}
    for w in dictionary:
        node = trie
        for ch in w:
            node = node.setdefault(ch, {})
        node['#'] = True

    R, C = len(board), len(board[0])
    found = set()

    def dfs(r, c, node, cur, used):
        ch = board[r][c]
        if ch not in node:
            return
        nxt = node[ch]
        cur += ch
        if nxt.get('#'):
            found.add(cur)
        used.add((r, c))
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if dr == 0 and dc == 0:
                    continue
                nr, nc = r + dr, c + dc
                if 0 <= nr < R and 0 <= nc < C and (nr, nc) not in used:
                    dfs(nr, nc, nxt, cur, used)
        used.discard((r, c))

    for r in range(R):
        for c in range(C):
            dfs(r, c, trie, "", set())
    return sorted(found)


if __name__ == "__main__":
    board = [['o', 'a', 'a', 'n'],
             ['e', 't', 'a', 'e'],
             ['i', 'h', 'k', 'r'],
             ['i', 'f', 'l', 'v']]
    dictionary = ["oath", "pea", "eat", "rain"]
    print("[" + ", ".join(solve(board, dictionary)) + "]")
