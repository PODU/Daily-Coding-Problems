// Day 1176: Word search in a 2D board via DFS backtracking.
// Try each cell as a start, explore 4 neighbors, mark visited in-place.
// Time O(M*N*4^L), Space O(L) recursion (L = word length).

function exists(board, word) {
    const rows = board.length, cols = board[0].length;
    function dfs(k, i, j) {
        if (k === word.length) return true;
        if (i < 0 || i >= rows || j < 0 || j >= cols || board[i][j] !== word[k]) return false;
        const saved = board[i][j];
        board[i][j] = '#';
        const found = dfs(k+1, i+1, j) || dfs(k+1, i-1, j) ||
                      dfs(k+1, i, j+1) || dfs(k+1, i, j-1);
        board[i][j] = saved;
        return found;
    }
    for (let i = 0; i < rows; i++)
        for (let j = 0; j < cols; j++)
            if (dfs(0, i, j)) return true;
    return false;
}

const board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']];
console.log(exists(board, "ABCCED") ? "true" : "false");
console.log(exists(board, "SEE")    ? "true" : "false");
console.log(exists(board, "ABCB")   ? "true" : "false");
