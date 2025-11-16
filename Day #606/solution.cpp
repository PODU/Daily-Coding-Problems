// Count all open knight's tours: DFS backtracking from every start square,
// counting Hamiltonian paths. Time O(8^(N*N)) worst, Space O(N*N). N=5 -> 1728.
#include <bits/stdc++.h>
using namespace std;

int N;
int dx[8] = {1, 1, -1, -1, 2, 2, -2, -2};
int dy[8] = {2, -2, 2, -2, 1, -1, 1, -1};

long long dfs(int x, int y, int visited, vector<vector<bool>>& board) {
    if (visited == N * N) return 1;
    long long count = 0;
    for (int k = 0; k < 8; ++k) {
        int nx = x + dx[k], ny = y + dy[k];
        if (nx >= 0 && nx < N && ny >= 0 && ny < N && !board[nx][ny]) {
            board[nx][ny] = true;
            count += dfs(nx, ny, visited + 1, board);
            board[nx][ny] = false;
        }
    }
    return count;
}

long long knightTours(int n) {
    N = n;
    if (n == 0) return 0;
    long long total = 0;
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j) {
            vector<vector<bool>> board(n, vector<bool>(n, false));
            board[i][j] = true;
            total += dfs(i, j, 1, board);
        }
    return total;
}

int main() {
    cout << knightTours(5) << endl;
    return 0;
}
