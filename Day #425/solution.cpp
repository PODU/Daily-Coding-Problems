// Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
// with blocking. O(1) board scan from the king's square.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> b = {
        "...K....",
        "........",
        ".B......",
        "......P.",
        ".......R",
        "..N.....",
        "........",
        ".....Q.."
    };
    int kr = -1, kc = -1;
    for (int i = 0; i < 8; i++)
        for (int j = 0; j < 8; j++)
            if (b[i][j] == 'K') { kr = i; kc = j; }

    auto in = [&](int r, int c) { return r >= 0 && r < 8 && c >= 0 && c < 8; };
    bool check = false;

    // Knight L-shapes
    int kn[8][2] = {{-2,-1},{-2,1},{-1,-2},{-1,2},{1,-2},{1,2},{2,-1},{2,1}};
    for (auto& d : kn) { int r = kr + d[0], c = kc + d[1]; if (in(r, c) && b[r][c] == 'N') check = true; }

    // White pawn attacks toward smaller row -> pawn sits at (kr+1, kc+-1)
    for (int dc = -1; dc <= 1; dc += 2) { int r = kr + 1, c = kc + dc; if (in(r, c) && b[r][c] == 'P') check = true; }

    // Rook / Queen orthogonal rays
    int orth[4][2] = {{-1,0},{1,0},{0,-1},{0,1}};
    for (auto& d : orth) {
        int r = kr + d[0], c = kc + d[1];
        while (in(r, c)) { char p = b[r][c]; if (p != '.') { if (p == 'R' || p == 'Q') check = true; break; } r += d[0]; c += d[1]; }
    }

    // Bishop / Queen diagonal rays
    int diag[4][2] = {{-1,-1},{-1,1},{1,-1},{1,1}};
    for (auto& d : diag) {
        int r = kr + d[0], c = kc + d[1];
        while (in(r, c)) { char p = b[r][c]; if (p != '.') { if (p == 'B' || p == 'Q') check = true; break; } r += d[0]; c += d[1]; }
    }

    cout << (check ? "True" : "False") << "\n";
    return 0;
}
