// Day 1184: Interleave ranked lists into one playlist respecting every ordering.
// Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
// Time O(V + E), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

vector<int> interleave(const vector<vector<int>>& lists) {
    vector<int> order;
    unordered_set<int> known;
    unordered_map<int, vector<int>> adj;
    unordered_map<int, int> indeg;
    set<pair<int,int>> edges;

    auto add = [&](int v) { if (!known.count(v)) { known.insert(v); order.push_back(v); indeg[v]; } };
    for (auto& l : lists) {
        for (int v : l) add(v);
        for (size_t i = 0; i + 1 < l.size(); i++) {
            int u = l[i], w = l[i + 1];
            if (u != w && !edges.count({u, w})) {
                edges.insert({u, w});
                adj[u].push_back(w);
                indeg[w]++;
            }
        }
    }

    deque<int> q;
    for (int v : order) if (indeg[v] == 0) q.push_back(v);
    vector<int> res;
    while (!q.empty()) {
        int v = q.front(); q.pop_front();
        res.push_back(v);
        for (int w : adj[v]) if (--indeg[w] == 0) q.push_back(w);
    }
    return res;
}

int main() {
    vector<vector<int>> lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
    auto r = interleave(lists);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [2, 1, 6, 7, 3, 9, 5]
    return 0;
}
