// Day 729: Interleave ranked preference lists into one consistent playlist.
// Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
// first-appearance tie-break). Time: O(V + E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

vector<int> interleave(vector<vector<int>>& lists) {
    unordered_map<int, vector<int>> adj;
    unordered_map<int, int> indeg;
    unordered_set<long long> edges;
    vector<int> order;
    auto ensure = [&](int x) {
        if (!indeg.count(x)) { indeg[x] = 0; adj[x] = {}; order.push_back(x); }
    };
    for (auto& lst : lists) {
        for (int x : lst) ensure(x);
        for (size_t i = 0; i + 1 < lst.size(); i++) {
            long long key = (long long)lst[i] * 1000000LL + lst[i + 1];
            if (!edges.count(key)) { edges.insert(key); adj[lst[i]].push_back(lst[i + 1]); indeg[lst[i + 1]]++; }
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
    vector<vector<int>> lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
    auto res = interleave(lists);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
