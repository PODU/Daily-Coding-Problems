// Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
// no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

bool isTree(int n, const vector<pair<int,int>>& edges) {
    if (n == 0) return true;
    vector<vector<int>> adj(n);
    for (size_t i = 0; i < edges.size(); i++) {
        int u = edges[i].first, v = edges[i].second;
        adj[u].push_back(v); adj[v].push_back(u);
    }
    vector<int> visited(n, 0);
    // iterative DFS storing (node, parent)
    stack<pair<int,int>> st; st.push({0, -1}); visited[0] = 1; int seen = 1;
    while (!st.empty()) {
        int u = st.top().first, p = st.top().second; st.pop();
        for (int w : adj[u]) {
            if (!visited[w]) { visited[w] = 1; seen++; st.push({w, u}); }
            else if (w != p) return false; // back-edge -> cycle
        }
    }
    return seen == n; // connected?
}

int main() {
    vector<pair<int,int>> tree = {{0,1},{1,2},{1,3}};
    vector<pair<int,int>> cyclic = {{0,1},{1,2},{2,0},{2,3}};
    cout << (isTree(4, tree) ? "True" : "False") << "\n";   // True
    cout << (isTree(4, cyclic) ? "True" : "False") << "\n"; // False
    return 0;
}
