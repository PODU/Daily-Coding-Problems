// Android unlock patterns of length N: DFS over the 1..9 keypad, tracking visited keys and a
// "skip" matrix (key jumped over must already be visited). Symmetry reduces work to 3 starts.
// Time: O(9!) worst case, Space: O(9).
#include <bits/stdc++.h>
using namespace std;

int skip[10][10];
bool visited[10];

int dfs(int cur, int remaining) {
    if (remaining == 0) return 1;
    visited[cur] = true;
    int count = 0;
    for (int next = 1; next <= 9; next++) {
        int mid = skip[cur][next];
        if (!visited[next] && (mid == 0 || visited[mid]))
            count += dfs(next, remaining - 1);
    }
    visited[cur] = false;
    return count;
}

int patterns(int n) {
    memset(skip, 0, sizeof(skip));
    skip[1][3] = skip[3][1] = 2;
    skip[1][7] = skip[7][1] = 4;
    skip[3][9] = skip[9][3] = 6;
    skip[7][9] = skip[9][7] = 8;
    skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5;
    skip[2][8] = skip[8][2] = 5;
    skip[4][6] = skip[6][4] = 5;
    // corner(1)*4, edge(2)*4, center(5)*1 by symmetry
    return dfs(1, n - 1) * 4 + dfs(2, n - 1) * 4 + dfs(5, n - 1);
}

int main() {
    for (int n = 1; n <= 9; n++) cout << "N=" << n << ": " << patterns(n) << "\n";
    // e.g. N=1 -> 9, N=4 -> 1624 (standard canonical counts)
}
