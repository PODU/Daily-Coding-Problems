# Day 1474: Boggle solver. Build a trie of the dictionary, then DFS from every
# cell over 8-adjacent neighbors (no reuse), pruning by trie prefixes.
# Time O(R*C*8^L) worst case with trie pruning; Space O(total dict chars).

def boggle(board, words):
    trie = {}
    for w in words:
        node = trie
        for ch in w:
            node = node.setdefault(ch, {})
        node['#'] = w

    rows, cols = len(board), len(board[0])
    found = set()

    def dfs(r, c, node):
        ch = board[r][c]
        if ch not in node:
            return
        nxt = node[ch]
        if '#' in nxt:
            found.add(nxt['#'])
        board[r][c] = '*'
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if dr == 0 and dc == 0:
                    continue
                nr, nc = r + dr, c + dc
                if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] != '*':
                    dfs(nr, nc, nxt)
        board[r][c] = ch

    for r in range(rows):
        for c in range(cols):
            dfs(r, c, trie)
    return sorted(found)


if __name__ == "__main__":
    board = [
        ['o', 'a', 'a', 'n'],
        ['e', 't', 'a', 'e'],
        ['i', 'h', 'k', 'r'],
        ['i', 'f', 'l', 'v'],
    ]
    words = ["oath", "pea", "eat", "rain"]
    print(boggle(board, words))  # ['eat', 'oath']
