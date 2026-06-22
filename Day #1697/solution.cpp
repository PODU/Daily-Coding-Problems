// Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
// Time O(N^2), Space O(N^2).
#include <iostream>
#include <vector>
#include <queue>
using namespace std;

bool isValidCrossword(const vector<vector<int>>& g) {
    int n = g.size();
    if (n == 0) return false;

    // 180-degree rotational symmetry
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (g[i][j] != g[n - 1 - i][n - 1 - j]) return false;

    // Run-length checks: each white cell needs horiz run >=3 and vert run >=3
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (g[i][j] != 0) continue;
            int l = j, r = j;
            while (l - 1 >= 0 && g[i][l - 1] == 0) l--;
            while (r + 1 < n && g[i][r + 1] == 0) r++;
            if (r - l + 1 < 3) return false;
            int t = i, b = i;
            while (t - 1 >= 0 && g[t - 1][j] == 0) t--;
            while (b + 1 < n && g[b + 1][j] == 0) b++;
            if (b - t + 1 < 3) return false;
        }
    }

    // Connectivity of white cells
    int total = 0, start = -1;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (g[i][j] == 0) { total++; if (start < 0) start = i * n + j; }
    if (total == 0) return false;

    vector<vector<bool>> vis(n, vector<bool>(n, false));
    queue<pair<int,int>> q;
    q.push(make_pair(start / n, start % n));
    vis[start / n][start % n] = true;
    int seen = 0;
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        pair<int,int> cur = q.front(); q.pop();
        int r = cur.first, c = cur.second;
        seen++;
        for (int d = 0; d < 4; d++) {
            int nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] && g[nr][nc] == 0) {
                vis[nr][nc] = true;
                q.push(make_pair(nr, nc));
            }
        }
    }
    return seen == total;
}

int main() {
    vector<vector<int>> valid(5, vector<int>(5, 0)); // all white
    cout << (isValidCrossword(valid) ? "true" : "false") << endl;

    vector<vector<int>> invalid(5, vector<int>(5, 0));
    invalid[0][0] = 1; // breaks symmetry
    cout << (isValidCrossword(invalid) ? "true" : "false") << endl;
    return 0;
}
