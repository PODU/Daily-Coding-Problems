// Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
// Returns a valid ordering, or empty (=> null) if a cycle exists. Time O(V+E), Space O(V+E).
#include <iostream>
#include <map>
#include <queue>
#include <string>
#include <vector>
using namespace std;

// prereqs[course] = list of its prerequisites.
// Returns {ok, ordering}. ok=false means a cycle exists (no valid ordering).
pair<bool, vector<string>> topoSort(const map<string, vector<string>>& prereqs) {
    map<string, vector<string>> adj;     // prereq -> courses depending on it
    map<string, int> indeg;
    for (auto& kv : prereqs) {
        indeg.emplace(kv.first, 0);
        for (auto& p : kv.second) indeg.emplace(p, 0);
    }
    for (auto& kv : prereqs) {
        for (auto& p : kv.second) {
            adj[p].push_back(kv.first);  // p must come before kv.first
            indeg[kv.first]++;
        }
    }
    priority_queue<string, vector<string>, greater<string>> q; // lexicographic for determinism
    for (auto& kv : indeg) if (kv.second == 0) q.push(kv.first);

    vector<string> order;
    while (!q.empty()) {
        string c = q.top(); q.pop();
        order.push_back(c);
        for (auto& nxt : adj[c])
            if (--indeg[nxt] == 0) q.push(nxt);
    }
    if (order.size() != indeg.size()) return {false, {}};
    return {true, order};
}

int main() {
    map<string, vector<string>> prereqs = {
        {"CSC300", {"CSC100", "CSC200"}},
        {"CSC200", {"CSC100"}},
        {"CSC100", {}},
    };
    auto res = topoSort(prereqs);
    if (!res.first) {
        cout << "null\n";
    } else {
        cout << "[";
        for (size_t i = 0; i < res.second.size(); ++i)
            cout << "'" << res.second[i] << "'" << (i + 1 < res.second.size() ? ", " : "");
        cout << "]\n";
    }
    return 0;
}
