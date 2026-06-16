// Day 1672: Determine if black king is in check on an 8x8 board.
// Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool inCheck(const vector<string>& b) {
    int kr = -1, kc = -1;
    for (int r = 0; r < 8; ++r)
        for (int c = 0; c < 8; ++c)
            if (b[r][c] == 'K') { kr = r; kc = c; }

    // Knight attacks
    int km[8][2] = {{-2,-1},{-2,1},{-1,-2},{-1,2},{1,-2},{1,2},{2,-1},{2,1}};
    for (auto& m : km) {
        int r = kr + m[0], c = kc + m[1];
        if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] == 'N') return true;
    }
    // Rays: dr,dc + flags (diag?)
    int dirs[8][2] = {{1,0},{-1,0},{0,1},{0,-1},{1,1},{1,-1},{-1,1},{-1,-1}};
    for (int d = 0; d < 8; ++d) {
        bool diag = (d >= 4);
        for (int step = 1; step < 8; ++step) {
            int r = kr + dirs[d][0]*step, c = kc + dirs[d][1]*step;
            if (r < 0 || r >= 8 || c < 0 || c >= 8) break;
            char p = b[r][c];
            if (p == '.') continue;
            if (diag) {
                if (p == 'B' || p == 'Q') return true;
                if (step == 1 && p == 'K') return true;
                // white pawn attacks upward: pawn below king on a down-diagonal
                if (step == 1 && p == 'P' && dirs[d][0] == 1) return true;
            } else {
                if (p == 'R' || p == 'Q') return true;
                if (step == 1 && p == 'K') return true;
            }
            break; // blocked by first piece
        }
    }
    return false;
}

int main() {
    vector<string> board = {
        "...K....","........",".B......","......P.",
        ".......R","..N.....","........",".....Q.."};
    cout << (inCheck(board) ? "True" : "False") << "\n"; // True
    return 0;
}
