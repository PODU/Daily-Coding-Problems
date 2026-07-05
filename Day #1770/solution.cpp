// Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
// Time: O(rows*cols), Space: O(rows*cols) for the queue/visited in worst case.
#include <bits/stdc++.h>
using namespace std;

void floodFill(vector<vector<char>>& g, int sr, int sc, char color) {
    int R = g.size(), C = g[0].size();
    char target = g[sr][sc];
    if (target == color) return;
    queue<pair<int,int>> q;
    q.push({sr, sc});
    g[sr][sc] = color;
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        auto [r, c] = q.front(); q.pop();
        for (int k = 0; k < 4; k++) {
            int nr = r + dr[k], nc = c + dc[k];
            if (nr >= 0 && nr < R && nc >= 0 && nc < C && g[nr][nc] == target) {
                g[nr][nc] = color;
                q.push({nr, nc});
            }
        }
    }
}

int main() {
    vector<vector<char>> g = {
        {'B','B','W'},
        {'W','W','W'},
        {'W','W','W'},
        {'B','B','B'}
    };
    floodFill(g, 2, 2, 'G');
    for (auto& row : g) {
        for (size_t j = 0; j < row.size(); j++) {
            cout << row[j];
            if (j + 1 < row.size()) cout << ' ';
        }
        cout << '\n';
    }
    return 0;
}
