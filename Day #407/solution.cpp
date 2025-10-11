// Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
// Approach: Prim with a min-heap over an undirected weighted graph.
// Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
#include <bits/stdc++.h>
using namespace std;

int minimumCost(const unordered_map<string, vector<pair<string,int>>>& adj) {
    if (adj.empty()) return 0;
    unordered_set<string> visited;
    // (cost, node)
    priority_queue<pair<int,string>, vector<pair<int,string>>, greater<>> pq;
    string start = adj.begin()->first;
    pq.push({0, start});
    int total = 0;
    while (!pq.empty()) {
        int cost = pq.top().first;
        string node = pq.top().second;
        pq.pop();
        if (visited.count(node)) continue;
        visited.insert(node);
        total += cost;
        auto it = adj.find(node);
        if (it != adj.end())
            for (const auto& e : it->second)
                if (!visited.count(e.first)) pq.push({e.second, e.first});
    }
    return total;
}

int main() {
    // Build undirected graph from the README's pipe configuration.
    vector<tuple<string,string,int>> edges = {
        {"plant","A",1}, {"plant","B",5}, {"plant","C",20},
        {"A","C",15}, {"B","C",10}
    };
    unordered_map<string, vector<pair<string,int>>> adj;
    // Ensure all nodes exist (including C with no outgoing pipes).
    for (string n : {"plant","A","B","C"}) adj[n];
    for (const auto& e : edges) {
        const string& u = get<0>(e);
        const string& v = get<1>(e);
        int w = get<2>(e);
        adj[u].push_back({v, w});
        adj[v].push_back({u, w});
    }
    cout << minimumCost(adj) << endl; // 16
    return 0;
}
