// Day 733: Connect 4 on a 7x6 grid.
// Approach: 2D board + per-column heights; after each drop scan 4 directions from the
// placed disc for 4-in-a-row. Time: O(1) per move (constant scan), Space: O(rows*cols).
#include <bits/stdc++.h>
using namespace std;

class Connect4 {
    static const int COLS = 7, ROWS = 6;
    vector<vector<char>> board;
    vector<int> height;
public:
    Connect4() : board(ROWS, vector<char>(COLS, '.')), height(COLS, 0) {}

    // returns true if this drop wins the game
    bool drop(int col, char color) {
        int r = height[col]++;
        board[r][col] = color;
        int dirs[4][2] = {{0,1},{1,0},{1,1},{1,-1}};
        for (auto& d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2)
                for (int k = 1; k < 4; k++) {
                    int nr = r + d[0]*k*s, nc = col + d[1]*k*s;
                    if (nr < 0 || nr >= ROWS || nc < 0 || nc >= COLS || board[nr][nc] != color) break;
                    cnt++;
                }
            if (cnt >= 4) return true;
        }
        return false;
    }

    void print() {
        for (int r = ROWS - 1; r >= 0; r--) {
            for (int c = 0; c < COLS; c++) cout << board[r][c] << (c + 1 < COLS ? ' ' : '\n');
        }
    }
};

int main() {
    Connect4 game;
    int moves[] = {0, 0, 1, 1, 2, 2, 3}; // R,B,R,B,R,B,R
    char color = 'R'; bool won = false;
    for (int m : moves) {
        won = game.drop(m, color);
        if (won) break;
        color = (color == 'R') ? 'B' : 'R';
    }
    game.print();
    cout << (won ? (color == 'R' ? "Red wins!" : "Black wins!") : "No winner yet") << "\n";
    return 0;
}
