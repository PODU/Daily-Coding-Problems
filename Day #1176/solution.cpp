// Day 1176: Word search in a 2D board via DFS backtracking.
// Try each cell as a start, explore 4 neighbors, mark visited in-place.
// Time O(M*N*4^L), Space O(L) recursion (L = word length).
#include <bits/stdc++.h>
using namespace std;

bool dfs(vector<vector<char>>& b, const string& w, int k, int i, int j) {
    if (k == (int)w.size()) return true;
    if (i < 0 || i >= (int)b.size() || j < 0 || j >= (int)b[0].size() || b[i][j] != w[k]) return false;
    char saved = b[i][j];
    b[i][j] = '#';                                // mark visited
    bool found = dfs(b, w, k+1, i+1, j) || dfs(b, w, k+1, i-1, j) ||
                 dfs(b, w, k+1, i, j+1) || dfs(b, w, k+1, i, j-1);
    b[i][j] = saved;
    return found;
}

bool exists(vector<vector<char>> b, const string& w) {
    for (int i = 0; i < (int)b.size(); i++)
        for (int j = 0; j < (int)b[0].size(); j++)
            if (dfs(b, w, 0, i, j)) return true;
    return false;
}

int main() {
    vector<vector<char>> board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
    cout << (exists(board, "ABCCED") ? "true" : "false") << "\n";
    cout << (exists(board, "SEE")    ? "true" : "false") << "\n";
    cout << (exists(board, "ABCB")   ? "true" : "false") << "\n";
    return 0;
}
