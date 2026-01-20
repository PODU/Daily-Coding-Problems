// Shortest uphill-then-downhill cycle from home (node 0).
// Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
#include <bits/stdc++.h>
using namespace std;

vector<long long> dijkstra(const vector<vector<pair<int,int>>>& adj, int src, int n) {
    const long long INF = LLONG_MAX;
    vector<long long> dist(n, INF);
    dist[src] = 0;
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    pq.push({0, src});
    while (!pq.empty()) {
        pair<long long,int> top = pq.top(); pq.pop();
        long long d = top.first; int u = top.second;
        if (d > dist[u]) continue;
        for (size_t j = 0; j < adj[u].size(); ++j) {
            int v = adj[u][j].first, w = adj[u][j].second;
            if (d + w < dist[v]) {
                dist[v] = d + w;
                pq.push({dist[v], v});
            }
        }
    }
    return dist;
}

int main() {
    map<int,int> elevations = {{0,5},{1,25},{2,15},{3,20},{4,10}};
    vector<tuple<int,int,int>> paths = {
        {0,1,10},{0,2,8},{0,3,15},{1,3,12},
        {2,4,10},{3,4,5},{3,0,17},{4,0,10}};
    int n = 0;
    for (map<int,int>::iterator it = elevations.begin(); it != elevations.end(); ++it)
        n = max(n, it->first + 1);

    vector<vector<pair<int,int>>> up(n), downRev(n);
    for (size_t j = 0; j < paths.size(); ++j) {
        int u = get<0>(paths[j]), v = get<1>(paths[j]), w = get<2>(paths[j]);
        if (elevations[v] > elevations[u]) up[u].push_back({v,w});
        else if (elevations[v] < elevations[u]) downRev[v].push_back({u,w});
    }
    auto upD = dijkstra(up, 0, n);
    auto dnD = dijkstra(downRev, 0, n);
    const long long INF = LLONG_MAX;
    long long best = INF;
    for (int p = 1; p < n; ++p)
        if (upD[p] != INF && dnD[p] != INF)
            best = min(best, upD[p] + dnD[p]);
    cout << best << endl;
    return 0;
}
