// Day 219: Connect 4 (7 cols x 6 rows).
// Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
// Win check O(1) per move (constant directions/length); board O(W*H).
#include <bits/stdc++.h>
using namespace std;

class Connect4 {
    static const int W = 7, H = 6;
    vector<vector<char>> grid; // grid[r][c], r=0 bottom
    vector<int> height;        // next free row per column
public:
    Connect4() : grid(H, vector<char>(W, '.')), height(W, 0) {}

    bool drop(int col, char player, int& outRow) {
        if (col < 0 || col >= W || height[col] >= H) return false;
        int r = height[col]++;
        grid[r][col] = player;
        outRow = r;
        return true;
    }

    bool wins(int r, int c, char p) {
        static int dirs[4][2] = {{0, 1}, {1, 0}, {1, 1}, {1, -1}};
        for (auto& d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2) {
                int rr = r + d[0] * s, cc = c + d[1] * s;
                while (rr >= 0 && rr < H && cc >= 0 && cc < W && grid[rr][cc] == p) {
                    cnt++; rr += d[0] * s; cc += d[1] * s;
                }
            }
            if (cnt >= 4) return true;
        }
        return false;
    }
};

int main() {
    Connect4 g;
    int cols[] = {0, 1, 0, 1, 0, 1, 0};
    char players[] = {'R', 'B', 'R', 'B', 'R', 'B', 'R'};
    string winner;
    for (int i = 0; i < 7; i++) {
        int row;
        g.drop(cols[i], players[i], row);
        if (g.wins(row, cols[i], players[i])) { winner = players[i]; break; }
    }
    cout << "Player " << winner << " wins!" << endl; // R wins vertically in column 0
    return 0;
}
