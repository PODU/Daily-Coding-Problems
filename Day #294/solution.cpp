// Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
// to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;
const long long INF = LLONG_MAX;

vector<long long> dijkstra(int n, vector<vector<pair<int,int>>>& adj, int src) {
    vector<long long> d(n, INF);
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    d[src] = 0; pq.push({0, src});
    while (!pq.empty()) {
        long long du = pq.top().first; int u = pq.top().second; pq.pop();
        if (du > d[u]) continue;
        for (size_t i = 0; i < adj[u].size(); ++i) { int v = adj[u][i].first, w = adj[u][i].second; if (d[u] + w < d[v]) { d[v] = d[u] + w; pq.push({d[v], v}); } }
    }
    return d;
}

int main() {
    int n = 5;
    vector<int> elev = {5, 25, 15, 20, 10};
    vector<tuple<int,int,int>> paths = {
        {0,1,10},{0,2,8},{0,3,15},{1,3,12},{2,4,10},{3,4,5},{3,0,17},{4,0,10}};
    vector<vector<pair<int,int>>> up(n), downRev(n);
    for (auto &t : paths) {
        int u = get<0>(t), v = get<1>(t), w = get<2>(t);
        if (elev[v] > elev[u]) up[u].push_back({v, w});
        if (elev[v] < elev[u]) downRev[v].push_back({u, w}); // reversed for m->0 search
    }
    auto distUp = dijkstra(n, up, 0);
    auto distDown = dijkstra(n, downRev, 0);
    long long best = INF;
    for (int m = 1; m < n; ++m)
        if (distUp[m] != INF && distDown[m] != INF) best = min(best, distUp[m] + distDown[m]);
    cout << best << "\n";
    return 0;
}
