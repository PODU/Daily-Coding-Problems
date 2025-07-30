// Day 54: Sudoku solver via backtracking with bitmask row/col/box constraints.
// Worst case exponential; bitmasks make pruning fast. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int rows[9], cols[9], boxes[9];

bool solve(vector<vector<int>>& g, int cell) {
    if (cell == 81) return true;
    int r = cell / 9, c = cell % 9, b = (r / 3) * 3 + c / 3;
    if (g[r][c] != 0) return solve(g, cell + 1);
    for (int d = 1; d <= 9; d++) {
        int bit = 1 << d;
        if ((rows[r] | cols[c] | boxes[b]) & bit) continue;
        g[r][c] = d;
        rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
        if (solve(g, cell + 1)) return true;
        g[r][c] = 0;
        rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
    }
    return false;
}

int main() {
    vector<vector<int>> g = {
        {5,3,0, 0,7,0, 0,0,0},
        {6,0,0, 1,9,5, 0,0,0},
        {0,9,8, 0,0,0, 0,6,0},
        {8,0,0, 0,6,0, 0,0,3},
        {4,0,0, 8,0,3, 0,0,1},
        {7,0,0, 0,2,0, 0,0,6},
        {0,6,0, 0,0,0, 2,8,0},
        {0,0,0, 4,1,9, 0,0,5},
        {0,0,0, 0,8,0, 0,7,9}
    };
    for (int r = 0; r < 9; r++)
        for (int c = 0; c < 9; c++)
            if (g[r][c]) {
                int bit = 1 << g[r][c];
                rows[r] |= bit; cols[c] |= bit; boxes[(r/3)*3 + c/3] |= bit;
            }
    solve(g, 0);
    for (int r = 0; r < 9; r++) {
        for (int c = 0; c < 9; c++) cout << g[r][c];
        cout << "\n";
    }
    return 0;
}
