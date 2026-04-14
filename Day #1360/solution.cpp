// Count Android unlock patterns of length N via DFS backtracking with a skip
// (midpoint) table; symmetry over corners/edges. Time O(N!) worst, Space O(N).
#include <bits/stdc++.h>
using namespace std;

int skip[10][10];
bool visited[10];

int dfs(int cur, int remaining) {
    if (remaining == 0) return 1;
    visited[cur] = true;
    int count = 0;
    for (int next = 1; next <= 9; ++next) {
        if (!visited[next] && (skip[cur][next] == 0 || visited[skip[cur][next]])) {
            count += dfs(next, remaining - 1);
        }
    }
    visited[cur] = false;
    return count;
}

int countPatterns(int n) {
    memset(skip, 0, sizeof(skip));
    skip[1][3] = skip[3][1] = 2;
    skip[1][7] = skip[7][1] = 4;
    skip[3][9] = skip[9][3] = 6;
    skip[7][9] = skip[9][7] = 8;
    skip[1][9] = skip[9][1] = skip[2][8] = skip[8][2] = 5;
    skip[3][7] = skip[7][3] = skip[4][6] = skip[6][4] = 5;
    // symmetry: 1 represents corners (x4), 2 represents edges (x4), 5 unique
    int corner = dfs(1, n - 1);
    int edge = dfs(2, n - 1);
    int center = dfs(5, n - 1);
    return corner * 4 + edge * 4 + center;
}

int main() {
    cout << countPatterns(4) << "\n";
    return 0;
}
