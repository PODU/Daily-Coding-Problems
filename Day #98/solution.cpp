// Day 98: Word search via DFS backtracking from each cell, marking visited cells
// in place. O(M*N*4^L) time, O(L) recursion space.
#include <bits/stdc++.h>
using namespace std;

bool dfs(vector<vector<char>>& b, const string& w, int r, int c, int i) {
    if (i == (int)w.size()) return true;
    if (r < 0 || r >= (int)b.size() || c < 0 || c >= (int)b[0].size() || b[r][c] != w[i])
        return false;
    char saved = b[r][c];
    b[r][c] = '#';
    bool found = dfs(b, w, r + 1, c, i + 1) || dfs(b, w, r - 1, c, i + 1) ||
                 dfs(b, w, r, c + 1, i + 1) || dfs(b, w, r, c - 1, i + 1);
    b[r][c] = saved;
    return found;
}

bool exists(vector<vector<char>> b, const string& w) {
    for (int r = 0; r < (int)b.size(); r++)
        for (int c = 0; c < (int)b[0].size(); c++)
            if (dfs(b, w, r, c, 0)) return true;
    return false;
}

int main() {
    vector<vector<char>> board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
    cout << boolalpha;
    cout << exists(board, "ABCCED") << "\n"; // true
    cout << exists(board, "SEE") << "\n";    // true
    cout << exists(board, "ABCB") << "\n";   // false
    return 0;
}
