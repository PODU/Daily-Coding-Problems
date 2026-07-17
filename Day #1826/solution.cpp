// Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
// O((V+E) log V).
#include <bits/stdc++.h>
using namespace std;

int main(){
    // nodes labeled 0..N (treat undirected network)
    vector<array<int,3>> edges = {
        {0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}
    };
    int maxNode = 0;
    for(auto& e : edges) maxNode = max({maxNode, e[0], e[1]});
    int V = maxNode + 1;
    vector<vector<pair<int,int>>> adj(V);
    for(auto& e : edges){ adj[e[0]].push_back({e[1], e[2]}); adj[e[1]].push_back({e[0], e[2]}); }

    vector<long long> dist(V, LLONG_MAX);
    priority_queue<pair<long long,int>, vector<pair<long long,int>>, greater<>> pq;
    dist[0] = 0; pq.push({0,0});
    while(!pq.empty()){
        auto top = pq.top(); pq.pop();
        long long d = top.first; int u = top.second;
        if(d > dist[u]) continue;
        for(auto& pr : adj[u]){
            int v = pr.first, w = pr.second;
            if(d + w < dist[v]){ dist[v] = d + w; pq.push({dist[v], v}); }
        }
    }
    long long ans = 0;
    for(int i = 0; i < V; i++) ans = max(ans, dist[i]);
    cout << ans << "\n"; // 9
    return 0;
}
