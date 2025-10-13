// Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
// M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
#include <bits/stdc++.h>
using namespace std;

int n;
vector<vector<int>> g, M;

void dfs(int src, int u) {
    M[src][u] = 1;
    for (int v : g[u])
        if (!M[src][v]) dfs(src, v);
}

int main() {
    g = {{0, 1, 3}, {1, 2}, {2}, {3}};
    n = g.size();
    M.assign(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dfs(i, i);
    for (int i = 0; i < n; i++) {
        cout << "[";
        for (int j = 0; j < n; j++) {
            cout << M[i][j];
            if (j + 1 < n) cout << ", ";
        }
        cout << "]\n";
    }
    return 0;
}
