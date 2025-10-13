// Day 427: Shortest uphill-then-downhill route from/to home (location 0).
// State Dijkstra: each node split into up/down phases; switch at the peak.
// Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<int, int> elev = {{0, 5}, {1, 25}, {2, 15}, {3, 20}, {4, 10}};
    vector<tuple<int, int, int>> edges = {
        {0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
        {2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}};
    int n = elev.size(), home = 0;
    vector<vector<pair<int, int>>> adj(n);
    for (auto& e : edges) {
        int u, v, w;
        tie(u, v, w) = e;
        adj[u].push_back({v, w});
    }
    int S = n * 2; // state = node*2 + phase (0 up, 1 down)
    vector<long long> dist(S, LLONG_MAX);
    vector<int> prev(S, -1);
    auto sid = [&](int node, int ph) { return node * 2 + ph; };
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<pair<long long, int>>> pq;
    dist[sid(home, 0)] = 0;
    pq.push(make_pair(0LL, sid(home, 0)));
    while (!pq.empty()) {
        long long d = pq.top().first;
        int s = pq.top().second;
        pq.pop();
        if (d > dist[s]) continue;
        int u = s / 2, ph = s % 2;
        if (ph == 0 && u != home) {
            int ns = sid(u, 1);
            if (d < dist[ns]) { dist[ns] = d; prev[ns] = s; pq.push(make_pair(d, ns)); }
        }
        for (auto& pr : adj[u]) {
            int v = pr.first, w = pr.second, ns;
            if (ph == 0 && elev[v] > elev[u]) ns = sid(v, 0);
            else if (ph == 1 && elev[v] < elev[u]) ns = sid(v, 1);
            else continue;
            if (d + w < dist[ns]) { dist[ns] = d + w; prev[ns] = s; pq.push(make_pair(d + w, ns)); }
        }
    }
    int goal = sid(home, 1);
    vector<int> nodes;
    for (int cur = goal; cur != -1; cur = prev[cur]) nodes.push_back(cur / 2);
    reverse(nodes.begin(), nodes.end());
    vector<int> path;
    for (int x : nodes)
        if (path.empty() || path.back() != x) path.push_back(x);
    for (size_t i = 0; i < path.size(); i++) {
        cout << path[i];
        if (i + 1 < path.size()) cout << " -> ";
    }
    cout << ", distance " << dist[goal] << endl;
    return 0;
}
