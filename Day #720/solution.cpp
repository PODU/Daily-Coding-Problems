// Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
// always filling the next empty cell. Time exponential worst-case but fast in practice.
#include <bits/stdc++.h>
using namespace std;

int rows[9], cols[9], boxes[9];
int grid[9][9];

int boxIdx(int r, int c) { return (r / 3) * 3 + c / 3; }

bool solve(int pos) {
    if (pos == 81) return true;
    int r = pos / 9, c = pos % 9;
    if (grid[r][c] != 0) return solve(pos + 1);
    int b = boxIdx(r, c);
    for (int d = 1; d <= 9; ++d) {
        int bit = 1 << d;
        if ((rows[r] | cols[c] | boxes[b]) & bit) continue;
        grid[r][c] = d; rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
        if (solve(pos + 1)) return true;
        grid[r][c] = 0; rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
    }
    return false;
}

int main() {
    string puzzle[9] = {
        "530070000", "600195000", "098000060",
        "800060003", "400803001", "700020006",
        "060000280", "000419005", "000080079"};
    for (int r = 0; r < 9; ++r)
        for (int c = 0; c < 9; ++c) {
            grid[r][c] = puzzle[r][c] - '0';
            if (grid[r][c]) {
                int bit = 1 << grid[r][c];
                rows[r] |= bit; cols[c] |= bit; boxes[boxIdx(r, c)] |= bit;
            }
        }
    solve(0);
    for (int r = 0; r < 9; ++r) {
        for (int c = 0; c < 9; ++c) cout << grid[r][c];
        cout << "\n";
    }
    return 0;
}
