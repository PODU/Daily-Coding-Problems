// Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
// Build row/column strings, substring search. Time O(N*M), Space O(N+M).
#include <bits/stdc++.h>
using namespace std;

bool findWord(const vector<string>& grid, const string& word) {
    int rows = grid.size(), cols = grid[0].size();
    for (int r = 0; r < rows; ++r)
        if (grid[r].find(word) != string::npos) return true;
    for (int c = 0; c < cols; ++c) {
        string col;
        for (int r = 0; r < rows; ++r) col += grid[r][c];
        if (col.find(word) != string::npos) return true;
    }
    return false;
}

int main() {
    vector<string> grid = {"FACI", "OBQP", "ANOB", "MASS"};
    cout << "'FOAM' -> " << (findWord(grid, "FOAM") ? "true" : "false") << "\n"; // true
    cout << "'MASS' -> " << (findWord(grid, "MASS") ? "true" : "false") << "\n"; // true
    return 0;
}
