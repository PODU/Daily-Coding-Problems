// Day 207: Check if an undirected graph is bipartite.
// BFS 2-coloring: color each component, fail if an edge joins same colors. Handles disconnected graphs.
// Time: O(V + E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

bool isBipartite(int n, vector<vector<int>>& adj) {
    vector<int> color(n, -1);
    for (int s = 0; s < n; ++s) {
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
    cout << boolalpha;
    // Square 0-1-2-3-0 (bipartite)
    vector<vector<int>> sq = {{1, 3}, {0, 2}, {1, 3}, {0, 2}};
    cout << isBipartite(4, sq) << endl; // true
    // Triangle 0-1-2-0 (not bipartite)
    vector<vector<int>> tri = {{1, 2}, {0, 2}, {0, 1}};
    cout << isBipartite(3, tri) << endl; // false
    return 0;
}
