// Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
// Walls are booleans (true=wall). Returns -1 if no path (null semantics).
#include <bits/stdc++.h>
using namespace std;

int shortestPath(const vector<vector<bool>>& grid, pair<int,int> start, pair<int,int> end) {
    int m = grid.size(), n = grid[0].size();
    if (grid[start.first][start.second] || grid[end.first][end.second]) return -1;
    vector<vector<bool>> visited(m, vector<bool>(n, false));
    queue<pair<int,int>> q;
    q.push(start);
    visited[start.first][start.second] = true;
    int steps = 0;
    int dr[] = {-1, 1, 0, 0};
    int dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        int sz = q.size();
        for (int i = 0; i < sz; i++) {
            int r = q.front().first, c = q.front().second; q.pop();
            if (r == end.first && c == end.second) return steps;
            for (int d = 0; d < 4; d++) {
                int nr = r + dr[d], nc = c + dc[d];
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !grid[nr][nc]) {
                    visited[nr][nc] = true;
                    q.push({nr, nc});
                }
            }
        }
        steps++;
    }
    return -1;
}

int main() {
    bool F = false, T = true;
    vector<vector<bool>> grid = {
        {F, F, F, F},
        {T, T, F, T},
        {F, F, F, F},
        {F, F, F, F}
    };
    cout << shortestPath(grid, {3, 0}, {0, 0}) << endl; // 7
    return 0;
}
