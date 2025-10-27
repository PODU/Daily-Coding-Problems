// BFS shortest path on a boolean grid (false=walkable, true=wall).
// Time O(M*N), Space O(M*N).
#include <bits/stdc++.h>
using namespace std;

int minSteps(const vector<vector<bool>>& board, pair<int,int> start, pair<int,int> end) {
    int M = board.size(), N = board[0].size();
    if (board[start.first][start.second] || board[end.first][end.second]) return -1;
    vector<vector<bool>> visited(M, vector<bool>(N, false));
    queue<pair<int,int>> q;
    q.push(start);
    visited[start.first][start.second] = true;
    int steps = 0;
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        int sz = q.size();
        for (int i = 0; i < sz; ++i) {
            auto [r, c] = q.front(); q.pop();
            if (r == end.first && c == end.second) return steps;
            for (int d = 0; d < 4; ++d) {
                int nr = r + dr[d], nc = c + dc[d];
                if (nr >= 0 && nr < M && nc >= 0 && nc < N && !visited[nr][nc] && !board[nr][nc]) {
                    visited[nr][nc] = true;
                    q.push({nr, nc});
                }
            }
        }
        ++steps;
    }
    return -1;
}

int main() {
    bool t = true, f = false;
    vector<vector<bool>> board = {
        {f, f, f, f},
        {t, t, f, t},
        {f, f, f, f},
        {f, f, f, f}
    };
    cout << minSteps(board, {3, 0}, {0, 0}) << endl;
    return 0;
}
