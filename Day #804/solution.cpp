// Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
// Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
// Time O((k+1) * E), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

struct Flight { string src, dst; int price; };

pair<int, vector<string>> cheapest(vector<Flight>& flights, string A, string B, int k) {
    const int INF = INT_MAX;
    unordered_map<string, int> dist;
    unordered_map<string, string> parent;
    for (auto& f : flights) { dist[f.src] = INF; dist[f.dst] = INF; }
    dist[A] = 0;
    for (int it = 0; it <= k; it++) {            // k+1 edges allowed
        auto snap = dist;
        for (auto& f : flights) {
            if (snap[f.src] == INF) continue;
            if (snap[f.src] + f.price < dist[f.dst]) {
                dist[f.dst] = snap[f.src] + f.price;
                parent[f.dst] = f.src;
            }
        }
    }
    if (dist[B] == INF) return {-1, {}};
    vector<string> path;
    for (string c = B; ; c = parent[c]) {
        path.push_back(c);
        if (c == A) break;
    }
    reverse(path.begin(), path.end());
    return {dist[B], path};
}

int main() {
    vector<Flight> flights = {
        {"JFK", "ATL", 150}, {"ATL", "SFO", 400}, {"ORD", "LAX", 200},
        {"LAX", "DFW", 80},  {"JFK", "HKG", 800}, {"ATL", "ORD", 90},
        {"JFK", "LAX", 500}};
    auto [cost, path] = cheapest(flights, "JFK", "LAX", 3);
    for (size_t i = 0; i < path.size(); i++)
        cout << path[i] << (i + 1 < path.size() ? " -> " : "");
    cout << ", costing $" << cost << "\n"; // JFK -> ATL -> ORD -> LAX, costing $440
    return 0;
}
