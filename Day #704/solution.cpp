// Day 704: Count valid Android unlock patterns of length N on a 3x3 keypad.
// Approach: DFS with a "skip" table (a jump is legal only if the middle key was
// already used); symmetry cuts the work. Time O(N!) worst but tiny (<=9 keys).
#include <bits/stdc++.h>
using namespace std;

int skipMid[10][10];
bool visited[10];

int dfs(int cur, int remaining) {
    if (remaining == 0) return 1;
    visited[cur] = true;
    int count = 0;
    for (int nx = 1; nx <= 9; ++nx) {
        if (!visited[nx]) {
            int mid = skipMid[cur][nx];
            if (mid == 0 || visited[mid]) count += dfs(nx, remaining - 1);
        }
    }
    visited[cur] = false;
    return count;
}

int numberOfPatterns(int N) {
    memset(skipMid, 0, sizeof(skipMid));
    skipMid[1][3] = skipMid[3][1] = 2;
    skipMid[1][7] = skipMid[7][1] = 4;
    skipMid[3][9] = skipMid[9][3] = 6;
    skipMid[7][9] = skipMid[9][7] = 8;
    skipMid[1][9] = skipMid[9][1] = skipMid[3][7] = skipMid[7][3] = 5;
    skipMid[2][8] = skipMid[8][2] = skipMid[4][6] = skipMid[6][4] = 5;
    // symmetry: corners (1) x4, edges (2) x4, center (5) x1
    return 4 * dfs(1, N - 1) + 4 * dfs(2, N - 1) + dfs(5, N - 1);
}

int main() {
    cout << numberOfPatterns(4) << "\n"; // 1624
    return 0;
}
