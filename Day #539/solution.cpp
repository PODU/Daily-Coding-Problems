// Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
// For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p, r;
    DSU(int n) : p(n), r(n, 0) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;            // already connected -> cycle
        if (r[a] < r[b]) swap(a, b);
        p[b] = a;
        if (r[a] == r[b]) r[a]++;
        return true;
    }
};

bool hasCycle(int n, const vector<pair<int,int>>& edges) {
    DSU d(n);
    for (auto& e : edges)
        if (!d.unite(e.first, e.second)) return true;
    return false;
}

int main() {
    vector<pair<int,int>> cyclic = {{0,1},{1,2},{2,3},{3,0}};
    vector<pair<int,int>> tree   = {{0,1},{1,2},{2,3}};
    cout << boolalpha << hasCycle(4, cyclic) << "\n";
    cout << boolalpha << hasCycle(4, tree)   << "\n";
    return 0;
}
