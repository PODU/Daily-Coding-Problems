// Shortest path on a boolean grid (true=wall) via BFS in 4 directions.
// Time: O(M*N), Space: O(M*N). Returns -1 (null) if unreachable.
#include <bits/stdc++.h>
using namespace std;

int shortestPath(const vector<vector<bool>>& board, pair<int,int> start, pair<int,int> end) {
    int m = board.size(), n = board[0].size();
    if (board[start.first][start.second] || board[end.first][end.second]) return -1;
    vector<vector<bool>> seen(m, vector<bool>(n, false));
    queue<pair<int,int>> q;
    q.push(start);
    seen[start.first][start.second] = true;
    int steps = 0;
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        int sz = q.size();
        for (int i = 0; i < sz; ++i) {
            int r = q.front().first, c = q.front().second; q.pop();
            if (r == end.first && c == end.second) return steps;
            for (int d = 0; d < 4; ++d) {
                int nr = r + dr[d], nc = c + dc[d];
                if (nr < 0 || nr >= m || nc < 0 || nc >= n) continue;
                if (seen[nr][nc] || board[nr][nc]) continue;
                seen[nr][nc] = true;
                q.push(make_pair(nr, nc));
            }
        }
        ++steps;
    }
    return -1;
}

int main() {
    vector<vector<bool>> board = {
        {false, false, false, false},
        {true,  true,  false, true },
        {false, false, false, false},
        {false, false, false, false}
    };
    int res = shortestPath(board, {3, 0}, {0, 0});
    if (res == -1) cout << "null\n";
    else cout << res << "\n";
    return 0;
}
