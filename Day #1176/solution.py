# Day 1176: Word search in a 2D board via DFS backtracking.
# Try each cell as a start, explore 4 neighbors, mark visited in-place.
# Time O(M*N*4^L), Space O(L) recursion (L = word length).


def exists(board, word):
    rows, cols = len(board), len(board[0])

    def dfs(k, i, j):
        if k == len(word):
            return True
        if i < 0 or i >= rows or j < 0 or j >= cols or board[i][j] != word[k]:
            return False
        saved, board[i][j] = board[i][j], '#'
        found = (dfs(k+1, i+1, j) or dfs(k+1, i-1, j) or
                 dfs(k+1, i, j+1) or dfs(k+1, i, j-1))
        board[i][j] = saved
        return found

    return any(dfs(0, i, j) for i in range(rows) for j in range(cols))


if __name__ == "__main__":
    board = [['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']]
    print("true" if exists(board, "ABCCED") else "false")
    print("true" if exists(board, "SEE") else "false")
    print("true" if exists(board, "ABCB") else "false")
