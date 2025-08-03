// Count open knight's tours on N x N board via backtracking DFS from every start.
// Time exponential, Space O(N^2).
#include <iostream>
#include <vector>
using namespace std;

int N;
const int dr[8] = {-2,-2,-1,-1,1,1,2,2};
const int dc[8] = {-1,1,-2,2,-2,2,-1,1};

long long dfs(vector<vector<bool>>& vis, int r, int c, int count) {
    if (count == N * N) return 1;
    long long total = 0;
    for (int k = 0; k < 8; ++k) {
        int nr = r + dr[k], nc = c + dc[k];
        if (nr >= 0 && nr < N && nc >= 0 && nc < N && !vis[nr][nc]) {
            vis[nr][nc] = true;
            total += dfs(vis, nr, nc, count + 1);
            vis[nr][nc] = false;
        }
    }
    return total;
}

long long countTours(int n) {
    N = n;
    long long total = 0;
    for (int r = 0; r < n; ++r)
        for (int c = 0; c < n; ++c) {
            vector<vector<bool>> vis(n, vector<bool>(n, false));
            vis[r][c] = true;
            total += dfs(vis, r, c, 1);
        }
    return total;
}

int main() {
    cout << countTours(5) << "\n";
    return 0;
}
