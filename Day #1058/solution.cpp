// Bipartite check via BFS 2-coloring over all components.
// Time: O(V + E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

bool isBipartite(const vector<vector<int>>& adj) {
    int n = adj.size();
    vector<int> color(n, -1);
    for (int s = 0; s < n; ++s) {
        if (color[s] != -1) continue;
        queue<int> q;
        q.push(s);
        color[s] = 0;
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : adj[u]) {
                if (color[v] == -1) {
                    color[v] = color[u] ^ 1;
                    q.push(v);
                } else if (color[v] == color[u]) {
                    return false;
                }
            }
        }
    }
    return true;
}

vector<vector<int>> build(int n, const vector<pair<int,int>>& edges) {
    vector<vector<int>> adj(n);
    for (auto& e : edges) {
        adj[e.first].push_back(e.second);
        adj[e.second].push_back(e.first);
    }
    return adj;
}

int main() {
    // 4-cycle 0-1-2-3-0 -> bipartite
    auto g1 = build(4, {{0,1},{1,2},{2,3},{3,0}});
    // triangle 0-1-2-0 -> not bipartite
    auto g2 = build(3, {{0,1},{1,2},{2,0}});
    cout << (isBipartite(g1) ? "true" : "false") << "\n";
    cout << (isBipartite(g2) ? "true" : "false") << "\n";
    return 0;
}
