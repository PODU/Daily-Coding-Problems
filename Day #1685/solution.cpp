// Day 1685: Merge ranked preference lists -> topological sort (Kahn's BFS, FIFO,
// first-seen tie-break). Each adjacent pair in a list is an edge. Time O(V+E).
#include <bits/stdc++.h>
using namespace std;

vector<int> interleave(const vector<vector<int>>& lists) {
    vector<int> order;                  // first-seen node order
    set<int> seen;
    map<int, vector<int>> adj;
    set<pair<int,int>> edges;
    map<int, int> indeg;

    auto touch = [&](int x) {
        if (!seen.count(x)) { seen.insert(x); order.push_back(x); indeg[x]; adj[x]; }
    };
    for (auto& lst : lists) {
        for (int x : lst) touch(x);
        for (size_t i = 0; i + 1 < lst.size(); ++i) {
            int a = lst[i], b = lst[i+1];
            if (!edges.count({a, b})) { edges.insert({a, b}); adj[a].push_back(b); indeg[b]++; }
        }
    }
    queue<int> q;
    for (int x : order) if (indeg[x] == 0) q.push(x);
    vector<int> res;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        res.push_back(u);
        for (int v : adj[u]) if (--indeg[v] == 0) q.push(v);
    }
    return res;
}

int main() {
    vector<vector<int>> lists = {{1,7,3},{2,1,6,7,9},{3,9,5}};
    auto r = interleave(lists);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [2, 1, 6, 7, 3, 9, 5]
    return 0;
}
