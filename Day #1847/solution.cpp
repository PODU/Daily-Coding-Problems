// Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
// Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.
#include <bits/stdc++.h>
using namespace std;

struct TwoSat {
    int n;
    vector<vector<int>> adj, adjT;
    vector<int> comp, order;
    vector<bool> used;
    vector<char> assign_;
    TwoSat(int vars) : n(vars), adj(2 * vars), adjT(2 * vars) {}

    // clause (lit u) OR (lit v), where each lit is encoded node id
    void addOr(int u, int v) {
        adj[u ^ 1].push_back(v); adjT[v].push_back(u ^ 1);
        adj[v ^ 1].push_back(u); adjT[u].push_back(v ^ 1);
    }
    void dfs1(int v) { used[v] = true; for (int to : adj[v]) if (!used[to]) dfs1(to); order.push_back(v); }
    void dfs2(int v, int c) { comp[v] = c; for (int to : adjT[v]) if (comp[to] == -1) dfs2(to, c); }

    bool solve() {
        used.assign(2 * n, false); order.clear();
        for (int i = 0; i < 2 * n; i++) if (!used[i]) dfs1(i);
        comp.assign(2 * n, -1);
        for (int i = 0, c = 0; i < 2 * n; i++) {
            int v = order[2 * n - 1 - i];
            if (comp[v] == -1) dfs2(v, c++);
        }
        assign_.assign(n, 0);
        for (int i = 0; i < n; i++) {
            if (comp[2 * i] == comp[2 * i + 1]) return false;
            assign_[i] = comp[2 * i] > comp[2 * i + 1];
        }
        return true;
    }
};

int main() {
    // vars a=0, b=1, c=2 ; lit(v,neg) = 2v+neg
    auto lit = [](int v, int neg) { return 2 * v + neg; };
    TwoSat ts(3);
    ts.addOr(lit(2, 1), lit(1, 0)); // (¬c OR b)
    ts.addOr(lit(1, 0), lit(2, 0)); // (b OR c)
    ts.addOr(lit(1, 1), lit(2, 0)); // (¬b OR c)
    ts.addOr(lit(2, 1), lit(0, 1)); // (¬c OR ¬a)

    if (!ts.solve()) { cout << "False\n"; return 0; }
    const char* names = "abc";
    for (int i = 0; i < 3; i++)
        cout << names[i] << " = " << (ts.assign_[i] ? "True" : "False") << (i < 2 ? ", " : "\n");
}
