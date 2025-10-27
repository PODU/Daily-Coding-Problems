// Validate American-style crossword grid: every white cell in a horizontal AND vertical run
// of length >=3, single connected component of white cells, 180-degree rotational symmetry.
// Time: O(N^2), Space: O(N^2).
#include <iostream>
#include <vector>
#include <queue>
using namespace std;

bool isCrossword(const vector<string>& g) {
    int n = g.size();
    if (n == 0) return false;
    auto white = [&](int i, int j) { return g[i][j] == '.'; };

    // Rule 4: rotational symmetry.
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (g[i][j] != g[n-1-i][n-1-j]) return false;

    // Rules 1 & 2: each white cell in horizontal and vertical run of length >= 3.
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (!white(i, j)) continue;
            int l = j; while (l > 0 && white(i, l-1)) l--;
            int r = j; while (r < n-1 && white(i, r+1)) r++;
            if (r - l + 1 < 3) return false;
            int t = i; while (t > 0 && white(t-1, j)) t--;
            int b = i; while (b < n-1 && white(b+1, j)) b++;
            if (b - t + 1 < 3) return false;
        }
    }

    // Rule 3: connectivity via BFS.
    int total = 0, start = -1;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (white(i, j)) { total++; if (start < 0) start = i*n+j; }
    if (total == 0) return true;
    vector<vector<bool>> seen(n, vector<bool>(n, false));
    queue<pair<int,int>> q;
    q.push({start/n, start%n});
    seen[start/n][start%n] = true;
    int cnt = 0;
    int dx[] = {0,0,1,-1}, dy[] = {1,-1,0,0};
    while (!q.empty()) {
        auto [x,y] = q.front(); q.pop(); cnt++;
        for (int d = 0; d < 4; d++) {
            int nx = x+dx[d], ny = y+dy[d];
            if (nx>=0&&nx<n&&ny>=0&&ny<n&&white(nx,ny)&&!seen[nx][ny]) {
                seen[nx][ny] = true; q.push({nx,ny});
            }
        }
    }
    return cnt == total;
}

int main() {
    vector<string> valid = {".....", ".....", ".....", ".....", "....."};
    vector<string> invalid = {"..#..", ".....", "#...#", ".....", "..#.."};
    cout << (isCrossword(valid) ? "true" : "false") << '\n';
    cout << (isCrossword(invalid) ? "true" : "false") << '\n';
    return 0;
}
