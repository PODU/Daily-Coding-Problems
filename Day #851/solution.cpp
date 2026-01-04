// Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
// Union-Find: detect cycle and connectivity in O(E alpha(V)).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p, r;
    DSU(int n): p(n), r(n, 0){ iota(p.begin(), p.end(), 0); }
    int find(int x){ return p[x] == x ? x : p[x] = find(p[x]); }
    bool uni(int a, int b){
        a = find(a); b = find(b);
        if(a == b) return false;            // cycle
        if(r[a] < r[b]) swap(a, b);
        p[b] = a; if(r[a] == r[b]) r[a]++;
        return true;
    }
};

bool isMinimallyConnected(int n, const vector<pair<int,int>>& edges){
    if((int)edges.size() != n - 1) return false;   // tree needs exactly n-1 edges
    DSU d(n);
    for(auto& e : edges) if(!d.uni(e.first, e.second)) return false; // cycle => not minimal
    int comp = 0;
    for(int i = 0; i < n; ++i) if(d.find(i) == i) comp++;
    return comp == 1;                              // connected
}

int main(){
    // a tree on 5 nodes
    cout << (isMinimallyConnected(5, {{0,1},{0,2},{1,3},{1,4}}) ? "True" : "False") << "\n"; // True
    // contains a cycle 0-1-2-0
    cout << (isMinimallyConnected(4, {{0,1},{1,2},{2,0},{2,3}}) ? "True" : "False") << "\n"; // False
    return 0;
}
