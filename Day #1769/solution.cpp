// Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
// states (node, phase). UP edges require rising elevation, DOWN edges require
// falling; a free phase switch (the peak) is allowed at non-home nodes.
// Time: O(E log V), Space: O(V+E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<int,int> elev = {{0,5},{1,25},{2,15},{3,20},{4,10}};
    vector<tuple<int,int,int>> paths = {
        {0,1,10},{0,2,8},{0,3,15},{1,3,12},
        {2,4,10},{3,4,5},{3,0,17},{4,0,10}
    };
    int n = elev.size();
    vector<vector<pair<int,int>>> adj(n);
    for (auto& [u,v,w] : paths) adj[u].push_back({v,w});

    // state = node*2 + phase (0 = up, 1 = down)
    const long long INF = LLONG_MAX;
    vector<long long> dist(2*n, INF);
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    dist[0*2+0] = 0;
    pq.push({0, 0*2+0});
    while (!pq.empty()) {
        auto [d, s] = pq.top(); pq.pop();
        if (d > dist[s]) continue;
        int node = s/2, phase = s%2;
        // free switch up->down at the peak (not at home, to avoid trivial route)
        if (phase == 0 && node != 0) {
            int ns = node*2+1;
            if (d < dist[ns]) { dist[ns] = d; pq.push({d, ns}); }
        }
        for (auto& [v,w] : adj[node]) {
            if (phase == 0 && elev[v] > elev[node]) {
                int ns = v*2+0;
                if (d+w < dist[ns]) { dist[ns]=d+w; pq.push({d+w, ns}); }
            }
            if (phase == 1 && elev[v] < elev[node]) {
                int ns = v*2+1;
                if (d+w < dist[ns]) { dist[ns]=d+w; pq.push({d+w, ns}); }
            }
        }
    }
    cout << dist[0*2+1] << "\n";
    return 0;
}
