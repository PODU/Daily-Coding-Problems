// Bipartite check via BFS 2-coloring. Time O(V+E), Space O(V).
#include <bits/stdc++.h>
using namespace std;

bool isBipartite(int n, vector<vector<int>>& adj) {
    vector<int> color(n, -1);
    for (int s = 0; s < n; s++) {
        if (color[s] != -1) continue;
        queue<int> q; q.push(s); color[s] = 0;
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : adj[u]) {
                if (color[v] == -1) { color[v] = color[u] ^ 1; q.push(v); }
                else if (color[v] == color[u]) return false;
            }
        }
    }
    return true;
}

int main() {
    auto build = [](int n, vector<pair<int,int>> edges) {
        vector<vector<int>> adj(n);
        for (auto& e : edges) { adj[e.first].push_back(e.second); adj[e.second].push_back(e.first); }
        return adj;
    };
    auto sq = build(4, {{0,1},{1,2},{2,3},{3,0}});   // even cycle -> bipartite
    auto tri = build(3, {{0,1},{1,2},{2,0}});         // odd cycle -> not bipartite
    cout << (isBipartite(4, sq) ? "true" : "false") << "\n";
    cout << (isBipartite(3, tri) ? "true" : "false") << "\n";
    return 0;
}
