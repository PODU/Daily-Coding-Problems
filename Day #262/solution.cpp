// Day 262: Find all bridges in a connected undirected graph.
// Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
// An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

#include <bits/stdc++.h>
using namespace std;

class BridgeFinder {
    int n, timer = 0;
    vector<vector<int>> adj;
    vector<int> disc, low;
    vector<pair<int,int>> bridges;

    void dfs(int u, int parent) {
        disc[u] = low[u] = ++timer;
        bool skippedParent = false;
        for (int v : adj[u]) {
            if (v == parent && !skippedParent) { skippedParent = true; continue; }
            if (disc[v] == 0) {
                dfs(v, u);
                low[u] = min(low[u], low[v]);
                if (low[v] > disc[u]) bridges.push_back({min(u, v), max(u, v)});
            } else {
                low[u] = min(low[u], disc[v]);
            }
        }
    }
public:
    BridgeFinder(int n) : n(n), adj(n), disc(n, 0), low(n, 0) {}
    void addEdge(int u, int v) { adj[u].push_back(v); adj[v].push_back(u); }
    vector<pair<int,int>> findBridges() {
        for (int i = 0; i < n; i++) if (disc[i] == 0) dfs(i, -1);
        sort(bridges.begin(), bridges.end());
        return bridges;
    }
};

int main() {
    BridgeFinder g(5);
    g.addEdge(0, 1); g.addEdge(1, 2); g.addEdge(2, 0);
    g.addEdge(1, 3); g.addEdge(3, 4);
    auto bridges = g.findBridges();
    cout << "Bridges: [";
    for (size_t i = 0; i < bridges.size(); i++) {
        cout << "(" << bridges[i].first << ", " << bridges[i].second << ")";
        if (i + 1 < bridges.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
