// Minimally-connected = tree: edges == n-1 AND connected. Use union-find/BFS.
// Time O(n + e), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool isTree(int n, const vector<pair<int,int>>& edges) {
    if ((int)edges.size() != n - 1) return false;
    vector<vector<int>> adj(n);
    for (auto& e : edges) {
        adj[e.first].push_back(e.second);
        adj[e.second].push_back(e.first);
    }
    vector<bool> visited(n, false);
    queue<int> q;
    q.push(0);
    visited[0] = true;
    int count = 1;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        for (int v : adj[u]) {
            if (!visited[v]) {
                visited[v] = true;
                count++;
                q.push(v);
            }
        }
    }
    return count == n;
}

int main() {
    int n = 4;
    vector<pair<int,int>> edges = {{0,1},{1,2},{1,3}};
    cout << (isTree(n, edges) ? "True" : "False") << "\n";
    return 0;
}
