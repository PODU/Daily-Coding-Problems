// Day 1278: Sudoku solver via backtracking with row/col/box bitmasks.
// Time: exponential worst-case but fast with constraint pruning. Space O(1).
#include <bits/stdc++.h>
using namespace std;

int g[9][9];
int rowm[9], colm[9], boxm[9];

inline int bidx(int r, int c) { return (r / 3) * 3 + c / 3; }

bool solve(int pos) {
    if (pos == 81) return true;
    int r = pos / 9, c = pos % 9;
    if (g[r][c] != 0) return solve(pos + 1);
    int used = rowm[r] | colm[c] | boxm[bidx(r, c)];
    for (int d = 1; d <= 9; ++d) {
        int bit = 1 << d;
        if (used & bit) continue;
        g[r][c] = d; rowm[r] |= bit; colm[c] |= bit; boxm[bidx(r, c)] |= bit;
        if (solve(pos + 1)) return true;
        g[r][c] = 0; rowm[r] &= ~bit; colm[c] &= ~bit; boxm[bidx(r, c)] &= ~bit;
    }
    return false;
}

int main() {
    const char* puzzle[9] = {
        "53..7....", "6..195...", ".98....6.",
        "8...6...3", "4..8.3..1", "7...2...6",
        ".6....28.", "...419..5", "....8..79"};
    for (int r = 0; r < 9; ++r)
        for (int c = 0; c < 9; ++c) {
            char ch = puzzle[r][c];
            int v = (ch == '.') ? 0 : ch - '0';
            g[r][c] = v;
            if (v) { int bit = 1 << v; rowm[r] |= bit; colm[c] |= bit; boxm[bidx(r, c)] |= bit; }
        }
    solve(0);
    for (int r = 0; r < 9; ++r) {
        string line;
        for (int c = 0; c < 9; ++c) line += char('0' + g[r][c]);
        cout << line << "\n";
    }
    return 0;
}
