// Count valid Android unlock patterns of length N via backtracking with a
// skip[a][b] over-jump table; use 8-fold symmetry of corners/edges/center.
// Time: O(N * 9!) worst-case bounded by symmetry; Space: O(9).
#include <bits/stdc++.h>
using namespace std;

int skip[10][10];
bool used[10];

int dfs(int cur, int remaining) {
    if (remaining == 0) return 1;
    used[cur] = true;
    int count = 0;
    for (int next = 1; next <= 9; next++) {
        if (used[next]) continue;
        int mid = skip[cur][next];
        if (mid == 0 || used[mid]) {
            count += dfs(next, remaining - 1);
        }
    }
    used[cur] = false;
    return count;
}

int countPatterns(int n) {
    memset(used, 0, sizeof(used));
    // symmetry: 4 corners (1), 4 edges (2), 1 center (5)
    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1);
}

int main() {
    memset(skip, 0, sizeof(skip));
    skip[1][3] = skip[3][1] = 2;
    skip[1][7] = skip[7][1] = 4;
    skip[3][9] = skip[9][3] = 6;
    skip[7][9] = skip[9][7] = 8;
    skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5;
    skip[2][8] = skip[8][2] = 5;
    skip[4][6] = skip[6][4] = 5;

    for (int n = 1; n <= 9; n++) {
        cout << "N=" << n << ": " << countPatterns(n) << "\n";
    }
    return 0;
}
