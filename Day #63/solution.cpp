// Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
// Time O(N*M*L), Space O(max(N,M)).
#include <iostream>
#include <string>
#include <vector>
using namespace std;

bool findWord(const vector<vector<char>>& grid, const string& word) {
    int n = grid.size(), m = grid[0].size();
    for (int r = 0; r < n; ++r) {
        string row;
        for (int c = 0; c < m; ++c) row += grid[r][c];
        if (row.find(word) != string::npos) return true;
    }
    for (int c = 0; c < m; ++c) {
        string col;
        for (int r = 0; r < n; ++r) col += grid[r][c];
        if (col.find(word) != string::npos) return true;
    }
    return false;
}

int main() {
    vector<vector<char>> grid = {
        {'F','A','C','I'},
        {'O','B','Q','P'},
        {'A','N','O','B'},
        {'M','A','S','S'}
    };
    cout << "'FOAM' -> " << (findWord(grid, "FOAM") ? "true" : "false") << "\n";
    cout << "'MASS' -> " << (findWord(grid, "MASS") ? "true" : "false") << "\n";
    return 0;
}
