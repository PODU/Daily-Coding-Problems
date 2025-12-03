// 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
// Time O(n + m), Space O(n + m).
#include <bits/stdc++.h>
using namespace std;

struct TwoSat {
    int n; // number of variables
    vector<vector<int>> g;
    vector<int> idx, low, comp;
    vector<bool> onstk;
    vector<int> stk;
    int cnt = 0, cc = 0;

    TwoSat(int vars) : n(vars), g(2 * vars), idx(2 * vars, -1),
                       low(2 * vars, 0), comp(2 * vars, -1),
                       onstk(2 * vars, false) {}

    int node(int var, bool sign) { return 2 * var + (sign ? 0 : 1); }
    int neg(int x) { return x ^ 1; }

    void addClause(int v1, bool s1, int v2, bool s2) {
        int x = node(v1, s1), y = node(v2, s2);
        g[neg(x)].push_back(y);
        g[neg(y)].push_back(x);
    }

    void tarjan(int start) {
        vector<pair<int, int>> work = {{start, 0}};
        while (!work.empty()) {
            int v = work.back().first, pi = work.back().second;
            if (pi == 0) {
                idx[v] = low[v] = cnt++;
                stk.push_back(v);
                onstk[v] = true;
            }
            bool recurse = false;
            int i = pi;
            for (; i < (int)g[v].size(); i++) {
                int w = g[v][i];
                if (idx[w] == -1) {
                    work.back().second = i + 1;
                    work.push_back({w, 0});
                    recurse = true;
                    break;
                } else if (onstk[w]) {
                    low[v] = min(low[v], idx[w]);
                }
            }
            if (recurse) continue;
            if (low[v] == idx[v]) {
                while (true) {
                    int w = stk.back(); stk.pop_back();
                    onstk[w] = false;
                    comp[w] = cc;
                    if (w == v) break;
                }
                cc++;
            }
            work.pop_back();
            if (!work.empty()) {
                int pv = work.back().first;
                low[pv] = min(low[pv], low[v]);
            }
        }
    }

    bool solve(vector<bool>& assign) {
        for (int i = 0; i < 2 * n; i++)
            if (idx[i] == -1) tarjan(i);
        assign.assign(n, false);
        for (int v = 0; v < n; v++) {
            if (comp[2 * v] == comp[2 * v + 1]) return false;
            assign[v] = comp[2 * v] < comp[2 * v + 1];
        }
        return true;
    }
};

int main() {
    // Variables a=0, b=1, c=2. sign true=positive, false=negated.
    // (!c OR b) AND (b OR c) AND (!b OR c) AND (!c OR !a)
    TwoSat ts(3);
    ts.addClause(2, false, 1, true);
    ts.addClause(1, true, 2, true);
    ts.addClause(1, false, 2, true);
    ts.addClause(2, false, 0, false);

    vector<bool> assign;
    if (!ts.solve(assign)) {
        cout << "UNSATISFIABLE" << endl;
        return 0;
    }
    const char* names[] = {"a", "b", "c"};
    for (int i = 0; i < 3; i++) {
        if (i) cout << ", ";
        cout << names[i] << " = " << (assign[i] ? "True" : "False");
    }
    cout << endl;
    return 0;
}
