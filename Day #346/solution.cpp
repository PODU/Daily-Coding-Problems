// Day 346: Cheapest itinerary with up to k connections.
// Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct Flight { string u, v; int w; };

pair<int, vector<string>> cheapest(vector<Flight>& fs, string src, string dst, int k) {
    map<string, pair<int, vector<string>>> best;
    best[src] = {0, {src}};
    for (int it = 0; it <= k; ++it) {
        auto nxt = best;
        for (auto& f : fs) {
            auto pit = best.find(f.u);
            if (pit == best.end()) continue;
            int cost = pit->second.first + f.w;
            auto nit = nxt.find(f.v);
            if (nit == nxt.end() || cost < nit->second.first) {
                auto path = pit->second.second;
                path.push_back(f.v);
                nxt[f.v] = {cost, path};
            }
        }
        best = nxt;
    }
    if (!best.count(dst)) return {-1, {}};
    return best[dst];
}

int main() {
    vector<Flight> flights = {
        {"JFK","ATL",150},{"ATL","SFO",400},{"ORD","LAX",200},
        {"LAX","DFW",80},{"JFK","HKG",800},{"ATL","ORD",90},{"JFK","LAX",500}
    };
    auto res = cheapest(flights, "JFK", "LAX", 3);
    for (size_t i = 0; i < res.second.size(); ++i)
        cout << (i ? " -> " : "") << res.second[i];
    cout << ", costing $" << res.first << ".\n";
    return 0;
}
