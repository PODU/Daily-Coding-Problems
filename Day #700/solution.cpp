// Day 700: Cheapest itinerary with at most k connections (k+1 flights).
// Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
// track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).
#include <bits/stdc++.h>
using namespace std;

pair<int, vector<string>> cheapest(vector<tuple<string,string,int>>& flights,
                                   string src, string dst, int k) {
    const int INF = INT_MAX;
    unordered_map<string,int> dist;
    unordered_map<string,string> par;
    dist[src] = 0;
    for (int it = 0; it <= k; ++it) {          // up to k+1 edges
        auto nd = dist; auto np = par;
        for (auto& [u, v, w] : flights) {
            if (dist.count(u)) {
                int cand = dist[u] + w;
                if (!nd.count(v) || cand < nd[v]) { nd[v] = cand; np[v] = u; }
            }
        }
        dist = nd; par = np;
    }
    if (!dist.count(dst)) return {-1, {}};
    vector<string> path; string cur = dst;
    while (cur != src) { path.push_back(cur); cur = par[cur]; }
    path.push_back(src);
    reverse(path.begin(), path.end());
    return {dist[dst], path};
}

int main() {
    vector<tuple<string,string,int>> flights = {
        {"JFK","ATL",150},{"ATL","SFO",400},{"ORD","LAX",200},{"LAX","DFW",80},
        {"JFK","HKG",800},{"ATL","ORD",90},{"JFK","LAX",500}};
    auto [cost, path] = cheapest(flights, "JFK", "LAX", 3);
    for (size_t i = 0; i < path.size(); ++i) cout << path[i] << (i + 1 < path.size() ? " -> " : "");
    cout << ", costing $" << cost << "\n"; // JFK -> ATL -> ORD -> LAX, costing $440
    return 0;
}
