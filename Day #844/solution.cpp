// Day 844: 2-SAT via implication graph + Tarjan SCC.
// Each clause (x OR y) adds edges !x->y and !y->x. Satisfiable iff no var shares an SCC with its negation.
// O(V+E). Variables 0..n-1; literal encoded as 2*v (true) / 2*v+1 (false/negated).
#include <bits/stdc++.h>
using namespace std;

struct TwoSat {
    int n;
    vector<vector<int>> adj;
    TwoSat(int vars): n(vars), adj(2*vars) {}
    int node(int var, bool neg){ return 2*var + (neg ? 1 : 0); }
    // clause: (lit a) OR (lit b); a/b given as (var, isNegated)
    void addClause(int va, bool na, int vb, bool nb){
        int a = node(va, na), b = node(vb, nb);
        adj[a^1].push_back(b); // !a -> b
        adj[b^1].push_back(a); // !b -> a
    }
    vector<int> comp, order, val; vector<bool> vis;
    void dfs1(int u){ vis[u]=true; for(int v: adj[u]) if(!vis[v]) dfs1(v); order.push_back(u); }
    vector<vector<int>> radj;
    void dfs2(int u, int c){ comp[u]=c; for(int v: radj[u]) if(comp[v]==-1) dfs2(v,c); }
    bool solve(vector<bool>& assign){
        vis.assign(2*n,false); order.clear();
        for(int i=0;i<2*n;i++) if(!vis[i]) dfs1(i);
        radj.assign(2*n,{});
        for(int u=0;u<2*n;u++) for(int v: adj[u]) radj[v].push_back(u);
        comp.assign(2*n,-1); int c=0;
        for(int i=2*n-1;i>=0;i--){ int u=order[i]; if(comp[u]==-1) dfs2(u,c++); }
        assign.assign(n,false);
        for(int v=0;v<n;v++){
            if(comp[2*v]==comp[2*v+1]) return false;
            assign[v] = comp[2*v] > comp[2*v+1]; // true literal has later SCC order
        }
        return true;
    }
};

int main(){
    // (¬c OR b) AND (b OR c) AND (¬b OR c) AND (¬c OR ¬a); vars a=0,b=1,c=2
    TwoSat ts(3);
    ts.addClause(2,true, 1,false);  // ¬c OR b
    ts.addClause(1,false,2,false);  // b OR c
    ts.addClause(1,true, 2,false);  // ¬b OR c
    ts.addClause(2,true, 0,true);   // ¬c OR ¬a
    vector<bool> assign;
    if(ts.solve(assign)){
        const char* nm = "abc";
        cout << "a = " << (assign[0]?"True":"False")
             << ", b = " << (assign[1]?"True":"False")
             << ", c = " << (assign[2]?"True":"False") << "\n";
    } else cout << "False\n";
    return 0;
}
