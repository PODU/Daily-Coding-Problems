# Day 1830: Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
def exists(board, word):
    R, C = len(board), len(board[0])

    def dfs(i, r, c):
        if i == len(word):
            return True
        if r < 0 or r >= R or c < 0 or c >= C or board[r][c] != word[i]:
            return False
        saved = board[r][c]
        board[r][c] = "#"
        found = (dfs(i + 1, r + 1, c) or dfs(i + 1, r - 1, c) or
                 dfs(i + 1, r, c + 1) or dfs(i + 1, r, c - 1))
        board[r][c] = saved
        return found

    return any(dfs(0, r, c) for r in range(R) for c in range(C))


if __name__ == "__main__":
    board = [
        ["A", "B", "C", "E"],
        ["S", "F", "C", "S"],
        ["A", "D", "E", "E"],
    ]
    for w in ["ABCCED", "SEE", "ABCB"]:
        print('exists(board, "{}") returns {}'.format(
            w, "true" if exists(board, w) else "false"))
