// Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
// Time: O(E log V), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

int networkDelay(int N, vector<array<int,3>>& edges) {
    int V = N + 1;
    vector<vector<pair<int,int>>> adj(V);
    for (auto& e : edges) {
        adj[e[0]].push_back({e[1], e[2]});
        adj[e[1]].push_back({e[0], e[2]});
    }
    const long long INF = LLONG_MAX;
    vector<long long> dist(V, INF);
    dist[0] = 0;
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    pq.push({0, 0});
    while (!pq.empty()) {
        long long d = pq.top().first;
        int u = pq.top().second;
        pq.pop();
        if (d > dist[u]) continue;
        for (size_t i = 0; i < adj[u].size(); i++) {
            int v = adj[u][i].first, w = adj[u][i].second;
            if (dist[u] + w < dist[v]) {
                dist[v] = dist[u] + w;
                pq.push({dist[v], v});
            }
        }
    }
    long long ans = 0;
    for (long long d : dist) if (d != INF) ans = max(ans, d);
    return (int)ans;
}

int main() {
    int N = 5;
    vector<array<int,3>> edges = {{0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}};
    cout << networkDelay(N, edges) << "\n";
    return 0;
}
