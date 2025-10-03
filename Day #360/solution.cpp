// Merge ranked lists via topological sort: edge a->b for consecutive a,b in any list; Kahn's with FIFO queue. O(V+E).
#include <bits/stdc++.h>
using namespace std;
vector<int> merge(const vector<vector<int>>& lists) {
    vector<int> order;                 // nodes in first-appearance order
    unordered_set<int> seen;
    map<int, vector<int>> adj;
    unordered_map<int, int> indeg;
    auto touch = [&](int x) {
        if (!seen.count(x)) { seen.insert(x); order.push_back(x); indeg[x]; }
    };
    for (auto& l : lists) {
        for (int x : l) touch(x);
        for (size_t i = 0; i + 1 < l.size(); i++) {
            adj[l[i]].push_back(l[i + 1]);
            indeg[l[i + 1]]++;
        }
    }
    deque<int> q;
    for (int x : order) if (indeg[x] == 0) q.push_back(x);
    vector<int> res;
    while (!q.empty()) {
        int u = q.front(); q.pop_front();
        res.push_back(u);
        for (int v : adj[u]) if (--indeg[v] == 0) q.push_back(v);
    }
    return res;
}
int main() {
    vector<vector<int>> lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
    vector<int> r = merge(lists);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) {
        if (i) cout << ", ";
        cout << r[i];
    }
    cout << "]\n";
    return 0;
}
