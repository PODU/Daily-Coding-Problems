// Day 1805: Find all bridges via Tarjan's algorithm (disc[]/low[], edge is bridge if low[v] > disc[u]).
// Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
#include <iostream>
#include <vector>
#include <algorithm>

struct Bridges {
    std::vector<std::vector<std::pair<int,int>>> adj; // (neighbor, edgeId)
    std::vector<int> disc, low;
    int timer = 0;
    std::vector<std::pair<int,int>> result;

    Bridges(int n) : adj(n), disc(n, -1), low(n, -1) {}

    void addEdge(int u, int v, int id) {
        adj[u].push_back({v, id});
        adj[v].push_back({u, id});
    }

    void dfs(int u, int parentEdge) {
        disc[u] = low[u] = timer++;
        for (size_t k = 0; k < adj[u].size(); ++k) {
            int v = adj[u][k].first, id = adj[u][k].second;
            if (id == parentEdge) continue;       // skip the single parent edge once
            if (disc[v] == -1) {
                dfs(v, id);
                low[u] = std::min(low[u], low[v]);
                if (low[v] > disc[u]) result.push_back({std::min(u,v), std::max(u,v)});
            } else {
                low[u] = std::min(low[u], disc[v]);
            }
        }
    }
};

int main() {
    int n = 5;
    Bridges b(n);
    int edges[][2] = {{0,1},{1,2},{2,0},{1,3},{3,4}};
    for (int i = 0; i < 5; ++i) b.addEdge(edges[i][0], edges[i][1], i);
    for (int i = 0; i < n; ++i) if (b.disc[i] == -1) b.dfs(i, -1);
    std::sort(b.result.begin(), b.result.end());
    for (size_t i = 0; i < b.result.size(); ++i) std::cout << b.result[i].first << " - " << b.result[i].second << "\n";
    return 0; // expected: 1 - 3 and 3 - 4
}
