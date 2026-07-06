// Day 1776: Count valid Android unlock patterns of length N.
// DFS with symmetry + jump-over rules. Time O(N!) bounded by 9!, Space O(9).
#include <bits/stdc++.h>
using namespace std;

int skipKey[10][10];
bool visited[10];

int dfs(int cur, int remaining) {
    if (remaining == 0) return 1;
    visited[cur] = true;
    int cnt = 0;
    for (int nxt = 1; nxt <= 9; ++nxt) {
        if (visited[nxt]) continue;
        int mid = skipKey[cur][nxt];
        if (mid == 0 || visited[mid]) cnt += dfs(nxt, remaining - 1);
    }
    visited[cur] = false;
    return cnt;
}

int countPatterns(int n) {
    // symmetry: corners(1,3,7,9), edges(2,4,6,8), center(5)
    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1);
}

int main() {
    memset(skipKey, 0, sizeof(skipKey));
    skipKey[1][3] = skipKey[3][1] = 2;
    skipKey[1][7] = skipKey[7][1] = 4;
    skipKey[3][9] = skipKey[9][3] = 6;
    skipKey[7][9] = skipKey[9][7] = 8;
    skipKey[1][9] = skipKey[9][1] = 5;
    skipKey[3][7] = skipKey[7][3] = 5;
    skipKey[2][8] = skipKey[8][2] = 5;
    skipKey[4][6] = skipKey[6][4] = 5;
    for (int n = 1; n <= 9; ++n)
        cout << "N=" << n << ": " << countPatterns(n) << "\n";
    return 0;
}
