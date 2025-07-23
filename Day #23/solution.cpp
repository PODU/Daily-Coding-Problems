// Shortest path on a grid with walls using BFS (4-directional).
// Time: O(M*N), Space: O(M*N).
#include <bits/stdc++.h>
using namespace std;

int shortestPath(const vector<vector<bool>>& board, pair<int,int> start, pair<int,int> end) {
    int m = board.size(), n = board[0].size();
    vector<vector<bool>> visited(m, vector<bool>(n, false));
    queue<tuple<int,int,int>> q;
    int dr[] = {-1, 1, 0, 0};
    int dc[] = {0, 0, -1, 1};
    q.push({start.first, start.second, 0});
    visited[start.first][start.second] = true;
    while (!q.empty()) {
        auto [r, c, d] = q.front(); q.pop();
        if (r == end.first && c == end.second) return d;
        for (int k = 0; k < 4; ++k) {
            int nr = r + dr[k], nc = c + dc[k];
            if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !board[nr][nc]) {
                visited[nr][nc] = true;
                q.push({nr, nc, d + 1});
            }
        }
    }
    return -1; // unreachable
}

int main() {
    bool f = false, t = true;
    vector<vector<bool>> board = {
        {f, f, f, f},
        {t, t, f, t},
        {f, f, f, f},
        {f, f, f, f}
    };
    int res = shortestPath(board, {3, 0}, {0, 0});
    if (res == -1) cout << "None" << endl;
    else cout << res << endl;
    return 0;
}
