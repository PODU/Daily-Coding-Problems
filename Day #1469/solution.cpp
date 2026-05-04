// Chess check detection: locate black king, test pawn/knight attacks and ray-cast
// bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.
#include <bits/stdc++.h>
using namespace std;

bool isWhite(char c) { return c == 'P' || c == 'N' || c == 'B' || c == 'R' || c == 'Q'; }

bool inCheck(const vector<string>& b) {
    int kr = -1, kc = -1;
    for (int r = 0; r < 8; r++)
        for (int c = 0; c < 8; c++)
            if (b[r][c] == 'K') { kr = r; kc = c; }
    if (kr < 0) return false;

    // White pawns move up (toward smaller row), attacking (r-1, c+-1) -> so a pawn
    // attacks the king if it sits at (kr+1, kc+-1).
    for (int dc : {-1, 1}) {
        int pr = kr + 1, pc = kc + dc;
        if (pr >= 0 && pr < 8 && pc >= 0 && pc < 8 && b[pr][pc] == 'P') return true;
    }

    // Knight moves.
    int km[8][2] = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
    for (auto& m : km) {
        int r = kr + m[0], c = kc + m[1];
        if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] == 'N') return true;
    }

    // Diagonal rays: bishop or queen.
    int diag[4][2] = {{1,1},{1,-1},{-1,1},{-1,-1}};
    for (auto& d : diag) {
        int r = kr + d[0], c = kc + d[1];
        while (r >= 0 && r < 8 && c >= 0 && c < 8) {
            char p = b[r][c];
            if (p != '.') { if (p == 'B' || p == 'Q') return true; break; }
            r += d[0]; c += d[1];
        }
    }

    // Straight rays: rook or queen.
    int strt[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    for (auto& d : strt) {
        int r = kr + d[0], c = kc + d[1];
        while (r >= 0 && r < 8 && c >= 0 && c < 8) {
            char p = b[r][c];
            if (p != '.') { if (p == 'R' || p == 'Q') return true; break; }
            r += d[0]; c += d[1];
        }
    }
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
    cout << (inCheck(board) ? "True" : "False") << "\n";
    return 0;
}
