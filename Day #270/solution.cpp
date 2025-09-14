// Day 270: Broadcast time = max shortest-path distance from node 0 (Dijkstra).
// Min-heap Dijkstra over undirected weighted graph; answer = max finite dist. Time O(E log V), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

int networkDelay(int n, const vector<array<int,3>>& edges) {
    vector<vector<pair<int,int>>> adj(n + 1);
    for (auto& e : edges) {
        adj[e[0]].push_back({e[1], e[2]});
        adj[e[1]].push_back({e[0], e[2]});
    }
    vector<long long> dist(n + 1, LLONG_MAX);
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    dist[0] = 0;
    pq.push({0, 0});
    while (!pq.empty()) {
        auto [d, u] = pq.top(); pq.pop();
        if (d > dist[u]) continue;
        for (auto& [v, w] : adj[u])
            if (d + w < dist[v]) { dist[v] = d + w; pq.push({dist[v], v}); }
    }
    long long ans = 0;
    for (int i = 0; i <= n; ++i) ans = max(ans, dist[i]);
    return (int)ans;
}

int main() {
    vector<array<int,3>> edges = {
        {0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}
    };
    cout << networkDelay(5, edges) << "\n";
    return 0;
}
