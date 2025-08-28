// Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
// BFS connectivity + edge count. Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

bool isMinimallyConnected(int v, const vector<pair<int,int>>& edges) {
    if ((int)edges.size() != v - 1) return false;            // tree must have V-1 edges
    vector<vector<int>> adj(v);
    for (auto& e : edges) { adj[e.first].push_back(e.second); adj[e.second].push_back(e.first); }
    vector<char> seen(v, 0);
    queue<int> q; q.push(0); seen[0] = 1; int cnt = 1;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        for (int w : adj[u]) if (!seen[w]) { seen[w] = 1; cnt++; q.push(w); }
    }
    return cnt == v;                                          // connected => with V-1 edges, acyclic
}

int main() {
    // A binary tree on 5 nodes: minimally-connected.
    vector<pair<int,int>> tree = {{0,1},{0,2},{1,3},{1,4}};
    // Same graph plus a cycle-forming edge: not minimally-connected.
    vector<pair<int,int>> cyclic = {{0,1},{0,2},{1,3},{1,4},{3,4}};
    cout << boolalpha << isMinimallyConnected(5, tree) << "\n";
    cout << boolalpha << isMinimallyConnected(5, cyclic) << "\n";
    return 0;
}
