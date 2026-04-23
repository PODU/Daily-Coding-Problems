// Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
// check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.
#include <bits/stdc++.h>
using namespace std;

class Connect4 {
public:
    static const int COLS = 7, ROWS = 6;
    vector<vector<char>> grid;
    char turn;
    Connect4() : grid(ROWS, vector<char>(COLS, '.')), turn('R') {}

    // returns 'R'/'B' if that move wins, ' ' otherwise, 'X' if column full/invalid
    char drop(int col) {
        if (col < 0 || col >= COLS || grid[ROWS - 1][col] != '.') return 'X';
        int r = 0;
        while (grid[r][col] != '.') r++;
        grid[r][col] = turn;
        char who = turn;
        turn = (turn == 'R') ? 'B' : 'R';
        return wins(r, col, who) ? who : ' ';
    }

    bool wins(int r, int c, char p) {
        int dirs[4][2] = {{0,1},{1,0},{1,1},{1,-1}};
        for (auto& d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2)
                for (int k = 1; k < 4; k++) {
                    int nr = r + d[0]*k*s, nc = c + d[1]*k*s;
                    if (nr<0||nr>=ROWS||nc<0||nc>=COLS||grid[nr][nc]!=p) break;
                    cnt++;
                }
            if (cnt >= 4) return true;
        }
        return false;
    }

    void print() {
        for (int r = ROWS - 1; r >= 0; r--) {
            for (int c = 0; c < COLS; c++) cout << grid[r][c] << ' ';
            cout << "\n";
        }
    }
};

int main() {
    Connect4 game;
    int moves[] = {0, 1, 0, 1, 0, 1, 0}; // Red wins vertically in column 0
    char winner = ' ';
    for (int m : moves) {
        char res = game.drop(m);
        if (res == 'R' || res == 'B') { winner = res; break; }
    }
    game.print();
    if (winner != ' ') cout << (winner == 'R' ? "Red" : "Black") << " wins!\n";
    else cout << "No winner yet.\n";
    return 0;
}
