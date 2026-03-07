// Day 1171: Validate an American-style crossword grid.
// Checks rotational symmetry, white-square connectivity (BFS), and that every
// maximal horizontal/vertical white run has length >= 3.
// Time O(N^2), Space O(N^2).  '#' = black, '.' = white.
#include <bits/stdc++.h>
using namespace std;

bool isValidCrossword(const vector<string>& g) {
    int n = (int)g.size();
    if (n == 0) return false;
    for (auto& r : g) if ((int)r.size() != n) return false;
    auto white = [&](int i, int j) { return g[i][j] == '.'; };

    // 1. Rotational symmetry.
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if ((g[i][j] == '.') != (g[n-1-i][n-1-j] == '.')) return false;

    // 2. Every horizontal white run must have length >= 3.
    for (int i = 0; i < n; i++) {
        int run = 0;
        for (int j = 0; j <= n; j++) {
            if (j < n && white(i, j)) run++;
            else { if (run > 0 && run < 3) return false; run = 0; }
        }
    }
    // 3. Every vertical white run must have length >= 3.
    for (int j = 0; j < n; j++) {
        int run = 0;
        for (int i = 0; i <= n; i++) {
            if (i < n && white(i, j)) run++;
            else { if (run > 0 && run < 3) return false; run = 0; }
        }
    }

    // 4. All white squares connected.
    int total = 0, si = -1, sj = -1;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (white(i, j)) { total++; if (si < 0) { si = i; sj = j; } }
    if (total == 0) return true;
    vector<vector<char>> vis(n, vector<char>(n, 0));
    queue<pair<int,int>> q; q.push(make_pair(si, sj)); vis[si][sj] = 1; int seen = 1;
    int dx[] = {0,0,1,-1}, dy[] = {1,-1,0,0};
    while (!q.empty()) {
        int x = q.front().first, y = q.front().second; q.pop();
        for (int d = 0; d < 4; d++) {
            int nx = x + dx[d], ny = y + dy[d];
            if (nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx][ny] && white(nx, ny)) {
                vis[nx][ny] = 1; seen++; q.push(make_pair(nx, ny));
            }
        }
    }
    return seen == total;
}

int main() {
    vector<string> g1 = {".....", ".....", ".....", ".....", "....."}; // valid
    vector<string> g2 = {".#...", ".....", ".....", ".....", "...#."}; // length-1 run
    cout << (isValidCrossword(g1) ? "true" : "false") << "\n";
    cout << (isValidCrossword(g2) ? "true" : "false") << "\n";
    return 0;
}
