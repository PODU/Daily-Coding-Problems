// Transitive closure: DFS from each vertex marking reachable nodes (incl self).
// Time O(V*(V+E)), Space O(V^2) for the matrix.
#include <bits/stdc++.h>
using namespace std;

void dfs(int u, const vector<vector<int>>& g, vector<int>& row) {
    row[u] = 1;
    for (int v : g[u]) if (!row[v]) dfs(v, g, row);
}

int main() {
    vector<vector<int>> graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
    int n = graph.size();
    vector<vector<int>> M(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dfs(i, graph, M[i]);

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
