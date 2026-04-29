// Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
// Approach: build each row and column string, check if target is a substring.
// Time: O(R*C*L) substring scan, Space: O(R+C).
#include <bits/stdc++.h>
using namespace std;

bool findWord(const vector<string>& grid, const string& target) {
    int rows = grid.size();
    if (rows == 0) return false;
    int cols = grid[0].size();
    // rows (left-to-right)
    for (const string& row : grid)
        if (row.find(target) != string::npos) return true;
    // columns (top-to-bottom)
    for (int c = 0; c < cols; ++c) {
        string col;
        for (int r = 0; r < rows; ++r) col += grid[r][c];
        if (col.find(target) != string::npos) return true;
    }
    return false;
}

int main() {
    vector<string> grid = {"FACI", "OBQP", "ANOB", "MASS"};
    cout << boolalpha;
    cout << findWord(grid, "FOAM") << endl; // true
    cout << findWord(grid, "MASS") << endl; // true
    return 0;
}
