// Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
// Returns -1 (null) if unreachable.
#include <bits/stdc++.h>
using namespace std;

int shortestPath(vector<vector<bool>>& g, pair<int,int> start, pair<int,int> end) {
    int m = g.size(), n = g[0].size();
    if (g[start.first][start.second] || g[end.first][end.second]) return -1;
    vector<vector<int>> dist(m, vector<int>(n, -1));
    queue<pair<int,int>> q;
    q.push(start); dist[start.first][start.second] = 0;
    int dx[] = {-1, 1, 0, 0}, dy[] = {0, 0, -1, 1};
    while (!q.empty()) {
        auto [r, c] = q.front(); q.pop();
        if (r == end.first && c == end.second) return dist[r][c];
        for (int k = 0; k < 4; k++) {
            int nr = r + dx[k], nc = c + dy[k];
            if (nr >= 0 && nr < m && nc >= 0 && nc < n && !g[nr][nc] && dist[nr][nc] == -1) {
                dist[nr][nc] = dist[r][c] + 1;
                q.push({nr, nc});
            }
        }
    }
    return -1;
}

int main() {
    bool t = true, f = false;
    vector<vector<bool>> g = {
        {f, f, f, f},
        {t, t, f, t},
        {f, f, f, f},
        {f, f, f, f}
    };
    int res = shortestPath(g, {3, 0}, {0, 0});
    if (res == -1) cout << "null\n"; else cout << res << "\n";
    return 0;
}
