// King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
// Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool inBoard(int r, int c) { return r >= 0 && r < 8 && c >= 0 && c < 8; }

bool isCheck(const vector<string>& b) {
    int kr = -1, kc = -1;
    for (int r = 0; r < 8; ++r)
        for (int c = 0; c < 8; ++c)
            if (b[r][c] == 'K') { kr = r; kc = c; }

    // Rook / Queen along rows and columns
    int dirsRC[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    for (auto& d : dirsRC) {
        int r = kr + d[0], c = kc + d[1];
        while (inBoard(r, c)) {
            char p = b[r][c];
            if (p != '.') { if (p == 'R' || p == 'Q') return true; break; }
            r += d[0]; c += d[1];
        }
    }
    // Bishop / Queen along diagonals
    int dirsD[4][2] = {{1,1},{1,-1},{-1,1},{-1,-1}};
    for (auto& d : dirsD) {
        int r = kr + d[0], c = kc + d[1];
        while (inBoard(r, c)) {
            char p = b[r][c];
            if (p != '.') { if (p == 'B' || p == 'Q') return true; break; }
            r += d[0]; c += d[1];
        }
    }
    // Knight
    int kn[8][2] = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
    for (auto& d : kn) {
        int r = kr + d[0], c = kc + d[1];
        if (inBoard(r, c) && b[r][c] == 'N') return true;
    }
    // White king adjacency
    for (int dr = -1; dr <= 1; ++dr)
        for (int dc = -1; dc <= 1; ++dc) {
            if (!dr && !dc) continue;
            int r = kr + dr, c = kc + dc;
            if (inBoard(r, c) && b[r][c] == 'K') return true;
        }
    // White pawn attacks upward toward smaller row, so it sits at kr+1 diagonally
    for (int dc = -1; dc <= 1; dc += 2) {
        int r = kr + 1, c = kc + dc;
        if (inBoard(r, c) && b[r][c] == 'P') return true;
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
    cout << (isCheck(board) ? "True" : "False") << endl; // True
    return 0;
}
