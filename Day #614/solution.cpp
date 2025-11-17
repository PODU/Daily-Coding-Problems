// Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
// Approach: Dijkstra from node 0, return the largest distance. Time O(E log V), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

long long broadcastTime(int n, vector<array<int,3>>& edges) {
    vector<vector<pair<int,int>>> adj(n + 1);
    for (auto& e : edges) adj[e[0]].push_back({e[1], e[2]});
    vector<long long> dist(n + 1, LLONG_MAX);
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    dist[0] = 0; pq.push({0, 0});
    while (!pq.empty()) {
        long long d = pq.top().first;
        int u = pq.top().second;
        pq.pop();
        if (d > dist[u]) continue;
        for (auto& nb : adj[u]) {
            int v = nb.first, w = nb.second;
            if (d + w < dist[v]) { dist[v] = d + w; pq.push({dist[v], v}); }
        }
    }
    long long ans = 0;
    for (int i = 0; i <= n; i++) ans = max(ans, dist[i]);
    return ans;
}

int main() {
    int N = 5;
    vector<array<int,3>> edges = {
        {0,1,5}, {0,2,3}, {0,5,4}, {1,3,8}, {2,3,1}, {3,5,10}, {3,4,5}
    };
    cout << broadcastTime(N, edges) << "\n"; // 9
    return 0;
}
