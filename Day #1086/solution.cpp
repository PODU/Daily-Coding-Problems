// Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
// Worst-case exponential, fast in practice; Space O(1).
#include <bits/stdc++.h>
using namespace std;

int rowM[9], colM[9], boxM[9];
int grid[9][9];

int boxId(int r, int c) { return (r / 3) * 3 + c / 3; }

bool solve(int pos) {
    if (pos == 81) return true;
    int r = pos / 9, c = pos % 9;
    if (grid[r][c] != 0) return solve(pos + 1);
    int b = boxId(r, c);
    for (int d = 1; d <= 9; d++) {
        int bit = 1 << d;
        if (!((rowM[r] | colM[c] | boxM[b]) & bit)) {
            grid[r][c] = d; rowM[r] |= bit; colM[c] |= bit; boxM[b] |= bit;
            if (solve(pos + 1)) return true;
            grid[r][c] = 0; rowM[r] &= ~bit; colM[c] &= ~bit; boxM[b] &= ~bit;
        }
    }
    return false;
}

int main() {
    int puzzle[9][9] = {
        {5,3,0,0,7,0,0,0,0},
        {6,0,0,1,9,5,0,0,0},
        {0,9,8,0,0,0,0,6,0},
        {8,0,0,0,6,0,0,0,3},
        {4,0,0,8,0,3,0,0,1},
        {7,0,0,0,2,0,0,0,6},
        {0,6,0,0,0,0,2,8,0},
        {0,0,0,4,1,9,0,0,5},
        {0,0,0,0,8,0,0,7,9}};
    for (int r = 0; r < 9; r++)
        for (int c = 0; c < 9; c++) {
            grid[r][c] = puzzle[r][c];
            if (puzzle[r][c]) {
                int bit = 1 << puzzle[r][c];
                rowM[r] |= bit; colM[c] |= bit; boxM[boxId(r, c)] |= bit;
            }
        }
    solve(0);
    for (int r = 0; r < 9; r++) { for (int c = 0; c < 9; c++) cout << grid[r][c]; cout << "\n"; }
}
