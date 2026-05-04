// Knight's tour counting via plain DFS backtracking from every start cell.
// Time: exponential (bounded by tour search); Space: O(N^2) visited grid + recursion.
#include <bits/stdc++.h>
using namespace std;

static const int DX[8] = {1, 1, -1, -1, 2, 2, -2, -2};
static const int DY[8] = {2, -2, 2, -2, 1, -1, 1, -1};

int N;
long long total;
vector<vector<char>> visited;

void dfs(int x, int y, int count) {
    if (count == N * N) { total++; return; }
    for (int d = 0; d < 8; d++) {
        int nx = x + DX[d], ny = y + DY[d];
        if (nx >= 0 && nx < N && ny >= 0 && ny < N && !visited[nx][ny]) {
            visited[nx][ny] = 1;
            dfs(nx, ny, count + 1);
            visited[nx][ny] = 0;
        }
    }
}

long long countTours(int n) {
    N = n; total = 0;
    visited.assign(n, vector<char>(n, 0));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) {
            visited[i][j] = 1;
            dfs(i, j, 1);
            visited[i][j] = 0;
        }
    return total;
}

int main() {
    cout << countTours(1) << "\n";
    cout << countTours(5) << "\n";
    return 0;
}
