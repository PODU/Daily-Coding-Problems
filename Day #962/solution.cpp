// Day 962: Count knight's tours on an N x N board (visit every square once).
// Approach: DFS backtracking from every start square. Time O(8^(N^2)) worst, Space O(N^2).
#include <bits/stdc++.h>
using namespace std;

int N;
int dx[8] = {1,1,-1,-1,2,2,-2,-2};
int dy[8] = {2,-2,2,-2,1,-1,1,-1};

long long dfs(vector<vector<bool>>& vis, int x, int y, int count) {
    if (count == N * N) return 1;
    long long total = 0;
    for (int d = 0; d < 8; ++d) {
        int nx = x + dx[d], ny = y + dy[d];
        if (nx >= 0 && nx < N && ny >= 0 && ny < N && !vis[nx][ny]) {
            vis[nx][ny] = true;
            total += dfs(vis, nx, ny, count + 1);
            vis[nx][ny] = false;
        }
    }
    return total;
}

long long countTours(int n) {
    N = n;
    long long total = 0;
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j) {
            vector<vector<bool>> vis(n, vector<bool>(n, false));
            vis[i][j] = true;
            total += dfs(vis, i, j, 1);
        }
    return total;
}

int main() {
    cout << countTours(5) << endl; // 1728
    return 0;
}
