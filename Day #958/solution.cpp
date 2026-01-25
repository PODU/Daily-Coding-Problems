// Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
// Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).
#include <bits/stdc++.h>
using namespace std;

vector<string> itinerary(vector<pair<string,string>> flights, string start) {
    map<string, multiset<string>> adj;
    for (auto& f : flights) adj[f.first].insert(f.second);
    int total = flights.size();
    vector<string> path = {start};
    function<bool(const string&)> dfs = [&](const string& node) -> bool {
        if ((int)path.size() == total + 1) return true;
        auto& dests = adj[node];
        for (auto it = dests.begin(); it != dests.end(); ++it) {
            string d = *it;
            it = dests.erase(it);
            path.push_back(d);
            if (dfs(d)) return true;
            path.pop_back();
            it = dests.insert(d);
        }
        return false;
    };
    if (dfs(start)) return path;
    return {};
}

void show(const vector<string>& v) {
    if (v.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) cout << "'" << v[i] << "'" << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    show(itinerary({{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL"));
    show(itinerary({{"SFO","COM"},{"COM","YYZ"}}, "COM"));
    show(itinerary({{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A"));
    return 0;
}
