// Day 1667: Course ordering via topological sort (Kahn's algorithm).
// Time O(V+E), Space O(V+E). Returns empty (null) if a cycle exists.
#include <bits/stdc++.h>
using namespace std;

vector<string> courseOrder(map<string, vector<string>>& prereqs) {
    // Build graph: edge prereq -> course. indegree = number of prereqs.
    map<string, int> indeg;
    map<string, vector<string>> adj;
    for (auto& [course, deps] : prereqs) {
        indeg.emplace(course, 0);
        for (auto& d : deps) { indeg.emplace(d, 0); }
    }
    for (auto& [course, deps] : prereqs)
        for (auto& d : deps) { adj[d].push_back(course); indeg[course]++; }

    priority_queue<string, vector<string>, greater<string>> pq; // sorted output
    for (auto& [c, d] : indeg) if (d == 0) pq.push(c);

    vector<string> order;
    while (!pq.empty()) {
        string c = pq.top(); pq.pop();
        order.push_back(c);
        for (auto& nxt : adj[c]) if (--indeg[nxt] == 0) pq.push(nxt);
    }
    if (order.size() != indeg.size()) return {}; // cycle -> null
    return order;
}

int main() {
    map<string, vector<string>> prereqs = {
        {"CSC300", {"CSC100", "CSC200"}},
        {"CSC200", {"CSC100"}},
        {"CSC100", {}}};
    auto order = courseOrder(prereqs);
    cout << "[";
    for (size_t i = 0; i < order.size(); i++)
        cout << "'" << order[i] << "'" << (i + 1 < order.size() ? ", " : "");
    cout << "]\n"; // ['CSC100', 'CSC200', 'CSC300']
    return 0;
}
