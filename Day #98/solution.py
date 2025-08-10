# Day 98: Word search via DFS backtracking from each cell, marking visited cells
# in place. O(M*N*4^L) time, O(L) recursion space (L = word length).
def exists(board, word):
    rows, cols = len(board), len(board[0])

    def dfs(r, c, i):
        if i == len(word):
            return True
        if r < 0 or r >= rows or c < 0 or c >= cols or board[r][c] != word[i]:
            return False
        board[r][c], saved = '#', board[r][c]  # mark visited
        found = (dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1)
                 or dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1))
        board[r][c] = saved
        return found

    return any(dfs(r, c, 0) for r in range(rows) for c in range(cols))


if __name__ == "__main__":
    board = [['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']]
    print(str(exists(board, "ABCCED")).lower())  # true
    print(str(exists(board, "SEE")).lower())     # true
    print(str(exists(board, "ABCB")).lower())    # false
