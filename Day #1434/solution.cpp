// Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
// Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
// Time: O((k+1) * E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

struct Flight { string src, dst; int price; };

string cheapest(const vector<Flight>& flights, const string& A, const string& B, int k) {
    unordered_map<string,int> dist;
    unordered_map<string,string> parent;
    for (auto& f : flights) { dist[f.src] = INT_MAX; dist[f.dst] = INT_MAX; }
    dist[A] = 0;
    // k connections -> at most k+1 edges
    for (int i = 0; i <= k; ++i) {
        unordered_map<string,int> cur = dist;
        unordered_map<string,string> curParent = parent;
        for (auto& f : flights) {
            if (dist[f.src] != INT_MAX && dist[f.src] + f.price < cur[f.dst]) {
                cur[f.dst] = dist[f.src] + f.price;
                curParent[f.dst] = f.src;
            }
        }
        dist = cur; parent = curParent;
    }
    if (dist[B] == INT_MAX) return "No route";
    // reconstruct
    vector<string> path;
    string node = B;
    while (node != A) { path.push_back(node); node = parent[node]; }
    path.push_back(A);
    reverse(path.begin(), path.end());
    string res;
    for (size_t i = 0; i < path.size(); ++i) {
        if (i) res += " -> ";
        res += path[i];
    }
    res += ", costing $" + to_string(dist[B]);
    return res;
}

int main() {
    vector<Flight> flights = {
        {"JFK","ATL",150}, {"ATL","SFO",400}, {"ORD","LAX",200},
        {"LAX","DFW",80}, {"JFK","HKG",800}, {"ATL","ORD",90}, {"JFK","LAX",500}
    };
    cout << cheapest(flights, "JFK", "LAX", 3) << endl;
    return 0;
}
