// Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
// DFS from each vertex marking everything reachable. O(V*(V+E)) time, O(V^2) space.
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> graph;
vector<vector<int>> M;

void dfs(int start, int u) {
    for (int v : graph[u]) {
        if (M[start][v] == 0) {
            M[start][v] = 1;
            dfs(start, v);
        }
    }
}

int main() {
    graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
    int n = graph.size();
    M.assign(n, vector<int>(n, 0));
    for (int s = 0; s < n; ++s) { M[s][s] = 1; dfs(s, s); }
    for (auto& row : M) {
        cout << '[';
        for (int i = 0; i < n; ++i) cout << row[i] << (i + 1 < n ? ", " : "");
        cout << "]\n";
    }
    return 0;
}
