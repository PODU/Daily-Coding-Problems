# Day 1009: Word Search: DFS backtracking from each cell, marking visited in-place.
# Time: O(M*N*4^L), Space: O(L) recursion depth.
def exists(board, word):
    rows, cols = len(board), len(board[0])

    def dfs(i, j, k):
        if k == len(word):
            return True
        if i < 0 or j < 0 or i >= rows or j >= cols or board[i][j] != word[k]:
            return False
        tmp = board[i][j]
        board[i][j] = '#'
        found = (dfs(i+1, j, k+1) or dfs(i-1, j, k+1) or
                 dfs(i, j+1, k+1) or dfs(i, j-1, k+1))
        board[i][j] = tmp
        return found

    for i in range(rows):
        for j in range(cols):
            if dfs(i, j, 0):
                return True
    return False


def main():
    board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']]
    for w in ("ABCCED", "SEE", "ABCB"):
        print(f"{w}: {str(exists([row[:] for row in board], w)).lower()}")


if __name__ == "__main__":
    main()
