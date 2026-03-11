// 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
// Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
// Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).
#include <bits/stdc++.h>
using namespace std;

int V;
vector<vector<int>> adj;
vector<int> comp, low, num, stk;
vector<bool> onStk;
int idx = 0, sccCount = 0;

int lit(int v, bool neg) { return 2 * v + (neg ? 1 : 0); }
void addEdge(int u, int v) { adj[u].push_back(v); }

void tarjan(int u) {
    low[u] = num[u] = idx++;
    stk.push_back(u); onStk[u] = true;
    for (int w : adj[u]) {
        if (num[w] == -1) { tarjan(w); low[u] = min(low[u], low[w]); }
        else if (onStk[w]) low[u] = min(low[u], num[w]);
    }
    if (low[u] == num[u]) {
        while (true) {
            int x = stk.back(); stk.pop_back(); onStk[x] = false;
            comp[x] = sccCount;
            if (x == u) break;
        }
        sccCount++;
    }
}

// clauses as ((var,neg),(var,neg))
vector<array<int,4>> clauses;
void clause(int v1, bool n1, int v2, bool n2) {
    addEdge(lit(v1, !n1), lit(v2, n2));
    addEdge(lit(v2, !n2), lit(v1, n1));
    clauses.push_back({v1, n1 ? 1 : 0, v2, n2 ? 1 : 0});
}

bool satisfies(const vector<bool>& val) {
    for (auto& c : clauses) {
        bool a = (val[c[0]] != (bool)c[1]); // literal value (neg flips)
        bool b = (val[c[2]] != (bool)c[3]);
        if (!(a || b)) return false;
    }
    return true;
}

int main() {
    V = 3; // a=0, b=1, c=2
    adj.assign(2 * V, {});
    comp.assign(2 * V, -1); low.assign(2 * V, 0);
    num.assign(2 * V, -1); onStk.assign(2 * V, false);

    // (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
    clause(2, true, 1, false);
    clause(1, false, 2, false);
    clause(1, true, 2, false);
    clause(2, true, 0, true);

    for (int i = 0; i < 2 * V; ++i)
        if (num[i] == -1) tarjan(i);

    for (int v = 0; v < V; ++v)
        if (comp[lit(v, false)] == comp[lit(v, true)]) {
            cout << "UNSATISFIABLE" << '\n';
            return 0;
        }

    vector<bool> val(V);
    for (int v = 0; v < V; ++v) val[v] = comp[lit(v, false)] < comp[lit(v, true)];
    if (!satisfies(val))
        for (int v = 0; v < V; ++v) val[v] = comp[lit(v, true)] < comp[lit(v, false)];

    const char* names[] = {"a", "b", "c"};
    cout << names[0] << " = " << (val[0] ? "True" : "False")
         << ", " << names[1] << " = " << (val[1] ? "True" : "False")
         << ", " << names[2] << " = " << (val[2] ? "True" : "False") << '\n';
    return 0;
}
