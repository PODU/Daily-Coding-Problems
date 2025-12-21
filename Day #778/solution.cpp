// Day 778: Interleave ranked preference lists -> topological sort (Kahn's).
// Consecutive items in each list become edges. FIFO queue + first-seen order.
// O(V + E).
#include <bits/stdc++.h>
using namespace std;

vector<int> interleave(const vector<vector<int>>& lists) {
    vector<int> order;                       // first-seen node order
    unordered_set<int> seen;
    unordered_map<int, vector<int>> adj;
    unordered_map<int, int> indeg;
    auto touch = [&](int x){ if (!seen.count(x)) { seen.insert(x); order.push_back(x); indeg[x]; } };
    for (auto& l : lists) {
        for (int x : l) touch(x);
        for (size_t i = 0; i + 1 < l.size(); i++) { adj[l[i]].push_back(l[i+1]); indeg[l[i+1]]++; }
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
    vector<vector<int>> lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
    auto r = interleave(lists);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [2, 1, 6, 7, 3, 9, 5]
    return 0;
}
