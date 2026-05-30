// Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
// drop() places a disc; checkWin scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).
#include <bits/stdc++.h>
using namespace std;

struct Connect4 {
    static const int COLS = 7, ROWS = 6;
    vector<vector<char>> g;   // '.' empty, 'R'/'B'
    Connect4() : g(ROWS, vector<char>(COLS, '.')) {}

    // returns row placed, or -1 if column full
    int drop(int col, char p) {
        for (int r = ROWS - 1; r >= 0; r--)
            if (g[r][col] == '.') { g[r][col] = p; return r; }
        return -1;
    }

    bool win(int r, int c) {
        char p = g[r][c];
        int dirs[4][2] = {{0,1},{1,0},{1,1},{1,-1}};
        for (auto& d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2) {
                int rr = r + s*d[0], cc = c + s*d[1];
                while (rr>=0&&rr<ROWS&&cc>=0&&cc<COLS&&g[rr][cc]==p) { cnt++; rr+=s*d[0]; cc+=s*d[1]; }
            }
            if (cnt >= 4) return true;
        }
        return false;
    }

    void print() {
        for (auto& row : g) { for (char ch : row) cout << ch << ' '; cout << "\n"; }
    }
};

int main() {
    Connect4 game;
    // R stacks column 0 four times vs B in column 1 -> R wins vertically
    int moves[][2] = {{0,'R'},{1,'B'},{0,'R'},{1,'B'},{0,'R'},{1,'B'},{0,'R'}};
    char winner = 0;
    for (auto& m : moves) {
        int r = game.drop(m[0], (char)m[1]);
        if (game.win(r, m[0])) { winner = (char)m[1]; break; }
    }
    game.print();
    cout << "Winner: " << (winner ? string(1, winner) : "none") << "\n"; // Winner: R
    return 0;
}
