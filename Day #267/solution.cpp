// Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
// rays and bishop/queen diagonal rays (stopping at the first blocker), and test
// knight and pawn attacks. Time O(8x8) = O(1); space O(1).
#include <bits/stdc++.h>
using namespace std;

bool inCheck(const vector<string>& board) {
    int kr = -1, kc = -1;
    for (int r = 0; r < 8; ++r)
        for (int c = 0; c < 8; ++c)
            if (board[r][c] == 'K') { kr = r; kc = c; }
    if (kr < 0) return false;

    auto at = [&](int r, int c) -> char {
        if (r < 0 || r >= 8 || c < 0 || c >= 8) return 0;
        return board[r][c];
    };

    // Orthogonal rays: Rook or Queen
    int orth[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    for (auto& d : orth) {
        int r = kr + d[0], c = kc + d[1];
        while (at(r, c) == '.') { r += d[0]; c += d[1]; }
        char p = at(r, c);
        if (p == 'R' || p == 'Q') return true;
    }
    // Diagonal rays: Bishop or Queen
    int diag[4][2] = {{1,1},{1,-1},{-1,1},{-1,-1}};
    for (auto& d : diag) {
        int r = kr + d[0], c = kc + d[1];
        while (at(r, c) == '.') { r += d[0]; c += d[1]; }
        char p = at(r, c);
        if (p == 'B' || p == 'Q') return true;
    }
    // Knight
    int kn[8][2] = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
    for (auto& d : kn)
        if (at(kr + d[0], kc + d[1]) == 'N') return true;
    // White pawn attacks the squares diagonally above it (row-1);
    // so a pawn at (kr+1, kc±1) attacks the king.
    if (at(kr + 1, kc - 1) == 'P' || at(kr + 1, kc + 1) == 'P') return true;
    // Enemy king adjacency
    for (int dr = -1; dr <= 1; ++dr)
        for (int dc = -1; dc <= 1; ++dc)
            if ((dr || dc) && at(kr + dr, kc + dc) == 'k') return true;
    return false;
}

int main() {
    vector<string> board = {
        "...K....",
        "........",
        ".B......",
        "......P.",
        ".......R",
        "..N.....",
        "........",
        ".....Q.."
    };
    cout << (inCheck(board) ? "True" : "False") << endl;
    return 0;
}
