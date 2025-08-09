// Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
// Returns a valid course order or empty (no order) if a cycle exists. O(V+E).
#include <bits/stdc++.h>
using namespace std;

vector<string> courseOrder(map<string, vector<string>>& prereqs, bool& ok) {
    map<string, int> indeg;
    map<string, vector<string>> adj;
    for (auto& kv : prereqs) indeg[kv.first];
    for (auto& kv : prereqs)
        for (auto& p : kv.second) { adj[p].push_back(kv.first); indeg[kv.first]++; }

    priority_queue<string, vector<string>, greater<>> q; // lexicographic
    for (auto& kv : indeg) if (kv.second == 0) q.push(kv.first);
    vector<string> order;
    while (!q.empty()) {
        string c = q.top(); q.pop();
        order.push_back(c);
        for (auto& n : adj[c]) if (--indeg[n] == 0) q.push(n);
    }
    ok = order.size() == prereqs.size();
    return order;
}

int main() {
    map<string, vector<string>> g = {
        {"CSC300", {"CSC100", "CSC200"}}, {"CSC200", {"CSC100"}}, {"CSC100", {}}};
    bool ok;
    auto order = courseOrder(g, ok);
    if (!ok) { cout << "null\n"; return 0; }
    cout << "[";
    for (size_t i = 0; i < order.size(); i++)
        cout << "'" << order[i] << "'" << (i + 1 < order.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
