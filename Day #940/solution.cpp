// Day 940: Time for a message from node 0 to reach all = max shortest-path distance (Dijkstra).
// Time O(E log V), Space O(V + E). Returns -1 if some node is unreachable.
#include <bits/stdc++.h>
using namespace std;

int networkDelay(int n, const vector<array<int,3>>& edges, int src) {
    vector<vector<pair<int,int>>> adj(n + 1); // node -> (neighbor, time)
    for (auto& e : edges) adj[e[0]].push_back({e[1], e[2]});
    vector<int> dist(n + 1, INT_MAX);
    dist[src] = 0;
    priority_queue<pair<int,int>, vector<pair<int,int>>, greater<>> pq;
    pq.push({0, src});
    while (!pq.empty()) {
        int d = pq.top().first, u = pq.top().second; pq.pop();
        if (d > dist[u]) continue;
        for (size_t i = 0; i < adj[u].size(); ++i) {
            int v = adj[u][i].first, w = adj[u][i].second;
            if (d + w < dist[v]) { dist[v] = d + w; pq.push({dist[v], v}); }
        }
    }
    int ans = 0;
    for (int i = 0; i <= n; ++i) {
        if (dist[i] == INT_MAX) return -1;
        ans = max(ans, dist[i]);
    }
    return ans;
}

int main() {
    int n = 5;
    vector<array<int,3>> edges = {
        {0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}};
    cout << networkDelay(n, edges, 0) << "\n"; // 9
    return 0;
}
