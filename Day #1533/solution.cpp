// Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
// Returns total cost to connect every house to the plant.
// Time O(E log V), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

int mstCost(map<string, map<string,int>>& g) {
    // build symmetric adjacency
    map<string, vector<pair<string,int>>> adj;
    for (auto& [u, nbrs] : g)
        for (auto& [v, w] : nbrs) {
            adj[u].push_back({v, w});
            adj[v].push_back({u, w});
        }
    set<string> visited;
    priority_queue<pair<int,string>, vector<pair<int,string>>, greater<>> pq;
    string start = g.begin()->first;
    pq.push({0, start});
    int total = 0;
    while (!pq.empty()) {
        auto [w, u] = pq.top(); pq.pop();
        if (visited.count(u)) continue;
        visited.insert(u);
        total += w;
        for (auto& [v, cw] : adj[u])
            if (!visited.count(v)) pq.push({cw, v});
    }
    return total;
}

int main() {
    map<string, map<string,int>> pipes = {
        {"plant", {{"A",1},{"B",5},{"C",20}}},
        {"A", {{"C",15}}},
        {"B", {{"C",10}}},
        {"C", {}}
    };
    cout << mstCost(pipes) << "\n";
    return 0;
}
