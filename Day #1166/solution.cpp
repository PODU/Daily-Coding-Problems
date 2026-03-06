// Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
// Time: O(rows*cols), Space: O(rows*cols).
#include <bits/stdc++.h>
using namespace std;

void floodFill(vector<vector<char>>& g, int sr, int sc, char color) {
    int rows = g.size(), cols = g[0].size();
    char start = g[sr][sc];
    if (start == color) return;
    queue<pair<int,int>> q;
    q.push({sr, sc});
    g[sr][sc] = color;
    int dr[4] = {-1,1,0,0}, dc[4] = {0,0,-1,1};
    while (!q.empty()) {
        auto [r, c] = q.front(); q.pop();
        for (int d = 0; d < 4; ++d) {
            int nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] == start) {
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
        for (size_t j = 0; j < row.size(); ++j) {
            if (j) cout << ' ';
            cout << row[j];
        }
        cout << "\n";
    }
    return 0;
}
