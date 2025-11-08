// Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    DSU(int n) : p(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    bool unite(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra == rb) return false; // already connected -> cycle
        p[ra] = rb;
        return true;
    }
};

bool isMinimallyConnected(int V, const vector<pair<int,int>>& edges) {
    if ((int)edges.size() != V - 1) return false;
    DSU d(V);
    for (auto& e : edges)
        if (!d.unite(e.first, e.second)) return false; // cycle
    int root = d.find(0);
    for (int i = 1; i < V; ++i)
        if (d.find(i) != root) return false; // not connected
    return true;
}

int main() {
    cout << (isMinimallyConnected(4, {{0,1},{1,2},{2,3}}) ? "True" : "False") << "\n";
    cout << (isMinimallyConnected(4, {{0,1},{1,2},{2,3},{3,0}}) ? "True" : "False") << "\n";
    return 0;
}
