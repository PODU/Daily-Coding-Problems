// Day 1108: Detect a cycle in an undirected graph using Union-Find.
// If an edge connects two already-connected vertices, there's a cycle.
// Time: O(E * alpha(V)). Space: O(V).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p, r;
    DSU(int n): p(n), r(n,0) { iota(p.begin(), p.end(), 0); }
    int find(int x){ return p[x]==x ? x : p[x]=find(p[x]); }
    bool unite(int a, int b){
        a=find(a); b=find(b);
        if (a==b) return false;            // already connected -> cycle
        if (r[a]<r[b]) swap(a,b);
        p[b]=a; if (r[a]==r[b]) r[a]++;
        return true;
    }
};

bool hasCycle(int n, vector<pair<int,int>>& edges){
    DSU d(n);
    for (auto& e : edges) if (!d.unite(e.first, e.second)) return true;
    return false;
}

int main(){
    vector<pair<int,int>> g1 = {{0,1},{1,2},{2,0}};   // triangle -> cycle
    vector<pair<int,int>> g2 = {{0,1},{1,2},{2,3}};   // path -> no cycle
    cout << (hasCycle(3, g1) ? "true" : "false") << "\n"; // true
    cout << (hasCycle(4, g2) ? "true" : "false") << "\n"; // false
    return 0;
}
