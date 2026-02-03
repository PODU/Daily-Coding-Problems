// Word Search: DFS backtracking from each cell, marking visited in-place.
// Time: O(M*N*4^L), Space: O(L) recursion depth.
#include <bits/stdc++.h>
using namespace std;

bool dfs(vector<vector<char>>& b, const string& w, int i, int j, int k) {
    if (k == (int)w.size()) return true;
    if (i < 0 || j < 0 || i >= (int)b.size() || j >= (int)b[0].size() || b[i][j] != w[k])
        return false;
    char tmp = b[i][j];
    b[i][j] = '#';
    bool found = dfs(b, w, i+1, j, k+1) || dfs(b, w, i-1, j, k+1) ||
                 dfs(b, w, i, j+1, k+1) || dfs(b, w, i, j-1, k+1);
    b[i][j] = tmp;
    return found;
}

bool exists(vector<vector<char>> b, const string& w) {
    for (int i = 0; i < (int)b.size(); i++)
        for (int j = 0; j < (int)b[0].size(); j++)
            if (dfs(b, w, i, j, 0)) return true;
    return false;
}

int main() {
    vector<vector<char>> board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
    cout << "ABCCED: " << (exists(board, "ABCCED") ? "true" : "false") << "\n";
    cout << "SEE: "    << (exists(board, "SEE")    ? "true" : "false") << "\n";
    cout << "ABCB: "   << (exists(board, "ABCB")   ? "true" : "false") << "\n";
    return 0;
}
