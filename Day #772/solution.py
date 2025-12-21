# Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
# marking visited. O(W) to build trie; search bounded by trie/board size.


def solve_boggle(board, dictionary):
    trie = {}
    for w in dictionary:
        node = trie
        for ch in w:
            node = node.setdefault(ch, {})
        node['$'] = True

    R, C = len(board), len(board[0])
    out = set()

    def dfs(r, c, node, path):
        ch = board[r][c]
        if ch == '#' or ch not in node:
            return
        nxt = node[ch]
        path += ch
        if '$' in nxt:
            out.add(path)
        board[r] = board[r][:c] + '#' + board[r][c + 1:]
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                if dr == 0 and dc == 0:
                    continue
                nr, nc = r + dr, c + dc
                if 0 <= nr < R and 0 <= nc < C:
                    dfs(nr, nc, nxt, path)
        board[r] = board[r][:c] + ch + board[r][c + 1:]

    for r in range(R):
        for c in range(C):
            dfs(r, c, trie, "")
    return sorted(out)


if __name__ == "__main__":
    board = ["oaan", "etae", "ihkr", "iflv"]
    dictionary = ["oath", "pea", "eat", "rain"]
    print(" ".join(solve_boggle(board, dictionary)))  # eat oath
