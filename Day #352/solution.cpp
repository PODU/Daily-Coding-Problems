// Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).
#include <bits/stdc++.h>
using namespace std;

bool valid(const vector<string>& g) {
    int n = (int)g.size();
    // 1. rotational symmetry
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if ((g[i][j] == '#') != (g[n-1-i][n-1-j] == '#')) return false;
    // 2a. horizontal runs >= 3
    for (int i = 0; i < n; i++) {
        int run = 0;
        for (int j = 0; j <= n; j++) {
            if (j < n && g[i][j] == '.') run++;
            else { if (run > 0 && run < 3) return false; run = 0; }
        }
    }
    // 2b. vertical runs >= 3
    for (int j = 0; j < n; j++) {
        int run = 0;
        for (int i = 0; i <= n; i++) {
            if (i < n && g[i][j] == '.') run++;
            else { if (run > 0 && run < 3) return false; run = 0; }
        }
    }
    // 3. connectivity
    vector<pair<int,int>> whites;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (g[i][j] == '.') whites.push_back({i, j});
    if (whites.empty()) return true;
    vector<vector<bool>> seen(n, vector<bool>(n, false));
    queue<pair<int,int>> q;
    q.push(whites[0]); seen[whites[0].first][whites[0].second] = true;
    int cnt = 0;
    int dx[] = {1,-1,0,0}, dy[] = {0,0,1,-1};
    while (!q.empty()) {
        auto [i, j] = q.front(); q.pop(); cnt++;
        for (int d = 0; d < 4; d++) {
            int ni = i + dx[d], nj = j + dy[d];
            if (ni >= 0 && ni < n && nj >= 0 && nj < n && g[ni][nj] == '.' && !seen[ni][nj]) {
                seen[ni][nj] = true; q.push({ni, nj});
            }
        }
    }
    return cnt == (int)whites.size();
}

int main() {
    vector<string> gridA(5, ".....");
    vector<string> gridB = {"#....", ".....", ".....", ".....", "....."};
    cout << (valid(gridA) ? "true" : "false") << "\n";
    cout << (valid(gridB) ? "true" : "false") << "\n";
    return 0;
}
