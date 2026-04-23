// Transitive closure: each input row is [node, neighbors...]. DFS from every node
// marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).
#include <bits/stdc++.h>
using namespace std;

void dfs(int u, vector<vector<int>>& adj, vector<int>& row) {
    row[u] = 1;
    for (int v : adj[u]) if (!row[v]) dfs(v, adj, row);
}

vector<vector<int>> transitiveClosure(const vector<vector<int>>& graph) {
    int n = graph.size();
    vector<vector<int>> adj(n);
    for (auto& r : graph) {
        int node = r[0];
        for (size_t i = 1; i < r.size(); i++) adj[node].push_back(r[i]);
    }
    vector<vector<int>> M(n, vector<int>(n, 0));
    for (int i = 0; i < n; i++) dfs(i, adj, M[i]);
    return M;
}

int main() {
    vector<vector<int>> graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
    auto M = transitiveClosure(graph);
    for (auto& row : M) {
        cout << "[";
        for (size_t j = 0; j < row.size(); j++) cout << row[j] << (j + 1 < row.size() ? ", " : "");
        cout << "]\n";
    }
    return 0;
}
