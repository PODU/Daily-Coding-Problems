// Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian-path algorithm with min-heap adjacency. O(E log E).
#include <bits/stdc++.h>
using namespace std;

vector<string> findItinerary(vector<pair<string,string>>& flights, const string& start) {
    map<string, priority_queue<string, vector<string>, greater<string>>> graph;
    for (auto& f : flights) graph[f.first].push(f.second);

    vector<string> route;
    vector<string> stk = {start};
    while (!stk.empty()) {
        string u = stk.back();
        if (graph.count(u) && !graph[u].empty()) {
            string v = graph[u].top(); graph[u].pop();
            stk.push_back(v);
        } else {
            route.push_back(u);
            stk.pop_back();
        }
    }
    reverse(route.begin(), route.end());
    if ((int)route.size() != (int)flights.size() + 1) return {}; // no valid itinerary
    return route;
}

void printRoute(const vector<string>& res) {
    if (res.empty()) { cout << "null\n"; return; } // no valid itinerary
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) cout << "'" << res[i] << "'" << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    vector<pair<string,string>> flights = {{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}};
    printRoute(findItinerary(flights, "YUL"));
    // ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
    vector<pair<string,string>> f2 = {{"SFO","COM"},{"COM","YYZ"}};
    printRoute(findItinerary(f2, "COM")); // null
    vector<pair<string,string>> f3 = {{"A","B"},{"A","C"},{"B","C"},{"C","A"}};
    printRoute(findItinerary(f3, "A"));
    // ['A', 'B', 'C', 'A', 'C']
    return 0;
}
