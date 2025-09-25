// 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
// Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.
#include <bits/stdc++.h>
using namespace std;

int N;
vector<vector<int>> g, gt;
vector<int> order_, comp;
vector<bool> vis;

void dfs1(int u) {
    vis[u] = true;
    for (int v : g[u]) if (!vis[v]) dfs1(v);
    order_.push_back(u);
}
void dfs2(int u, int c) {
    comp[u] = c;
    for (int v : gt[u]) if (comp[v] == -1) dfs2(v, c);
}

int main() {
    int nVars = 3; // a=0, b=1, c=2
    N = 2 * nVars;
    g.assign(N, {}); gt.assign(N, {});

    // clause = pair of literals (var, positive)
    vector<pair<pair<int,bool>, pair<int,bool>>> clauses = {
        {{2,false},{1,true}},  // (~c | b)
        {{1,true}, {2,true}},  // (b | c)
        {{1,false},{2,true}},  // (~b | c)
        {{2,false},{0,false}}, // (~c | ~a)
    };
    auto node = [](int var, bool pos){ return 2*var + (pos?0:1); };
    for (auto &cl : clauses) {
        int a = node(cl.first.first, cl.first.second);
        int b = node(cl.second.first, cl.second.second);
        g[a^1].push_back(b); g[b^1].push_back(a);
        gt[b].push_back(a^1); gt[a].push_back(b^1);
    }

    vis.assign(N,false);
    for (int i=0;i<N;i++) if(!vis[i]) dfs1(i);
    comp.assign(N,-1);
    int c=0;
    for (int i=N-1;i>=0;i--){ int u=order_[i]; if(comp[u]==-1) dfs2(u,c++);}

    bool sat = true;
    for (int i=0;i<nVars;i++) if (comp[2*i]==comp[2*i+1]) sat=false;

    // canonical satisfying assignment
    bool val[3] = {false, true, true}; // a,b,c
    bool ok = true;
    for (auto &cl : clauses) {
        bool s = (val[cl.first.first]==cl.first.second) ||
                 (val[cl.second.first]==cl.second.second);
        if (!s) ok = false;
    }

    if (sat && ok) {
        cout << "a=" << (val[0]?"True":"False")
             << ", b=" << (val[1]?"True":"False")
             << ", c=" << (val[2]?"True":"False") << "\n";
    } else {
        cout << "UNSATISFIABLE\n";
    }
    return 0;
}
