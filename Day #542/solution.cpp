// Bipartite check via BFS two-coloring. O(V+E) time, O(V) space.
#include <bits/stdc++.h>
using namespace std;

bool isBipartite(int n, const vector<vector<int>>& adj) {
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
    // Even cycle 0-1-2-3-0 -> bipartite
    vector<vector<int>> g1(4);
    int e1[][2] = {{0,1},{1,2},{2,3},{3,0}};
    for (auto& e : e1) { g1[e[0]].push_back(e[1]); g1[e[1]].push_back(e[0]); }
    cout << (isBipartite(4, g1) ? "true" : "false") << "\n";

    // Odd cycle 0-1-2-0 -> not bipartite
    vector<vector<int>> g2(3);
    int e2[][2] = {{0,1},{1,2},{2,0}};
    for (auto& e : e2) { g2[e[0]].push_back(e[1]); g2[e[1]].push_back(e[0]); }
    cout << (isBipartite(3, g2) ? "true" : "false") << "\n";
    return 0;
}
