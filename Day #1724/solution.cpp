// Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
// Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
// Time: O(k * E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<tuple<string, string, int>> flights = {
        {"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
        {"LAX", "DFW", 80},  {"JFK", "HKG", 800}, {"ATL", "ORD", 90},
        {"JFK", "LAX", 500},
    };
    string src = "JFK", dst = "LAX";
    int k = 3; // max flights (edges)

    const int INF = INT_MAX;
    unordered_map<string, int> dist;
    unordered_map<string, string> parent;
    set<string> cities;
    for (auto& f : flights) { cities.insert(get<0>(f)); cities.insert(get<1>(f)); }
    for (auto& c : cities) dist[c] = INF;
    dist[src] = 0;

    // Relax all edges k times over a snapshot to bound edge count.
    for (int i = 0; i < k; ++i) {
        auto snap = dist;
        for (auto& f : flights) {
            const string& u = get<0>(f); const string& v = get<1>(f); int w = get<2>(f);
            if (snap[u] != INF && snap[u] + w < dist[v]) {
                dist[v] = snap[u] + w;
                parent[v] = u;
            }
        }
    }

    if (dist[dst] == INF) { cout << "No route\n"; return 0; }
    vector<string> path;
    for (string at = dst; ; at = parent[at]) {
        path.push_back(at);
        if (at == src) break;
    }
    reverse(path.begin(), path.end());
    for (size_t i = 0; i < path.size(); ++i)
        cout << path[i] << (i + 1 < path.size() ? " -> " : "");
    cout << ", costing $" << dist[dst] << "\n";
    return 0;
}
