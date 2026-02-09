// Day 1053: Directional consistency. Decompose each rule into strict x/y
// inequalities, build a directed "greater-than" graph per axis, and detect a
// cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

#include <bits/stdc++.h>
using namespace std;

using Graph = map<string, set<string>>;

void add(Graph& g, const string& u, const string& v) {
    g[u].insert(v);
    g[v];
}

bool dfs(Graph& g, const string& u, map<string, int>& state) {
    state[u] = 0;
    for (auto& w : g[u]) {
        if (state.count(w) && state[w] == 0) return true;
        if (!state.count(w) && dfs(g, w, state)) return true;
    }
    state[u] = 1;
    return false;
}

bool hasCycle(Graph& g) {
    map<string, int> state;
    for (auto& kv : g)
        if (!state.count(kv.first) && dfs(g, kv.first, state)) return true;
    return false;
}

bool validate(const vector<string>& rules) {
    Graph gx, gy;
    for (auto& rule : rules) {
        istringstream iss(rule);
        string a, d, b; iss >> a >> d >> b;
        for (char ch : d) {
            if (ch == 'N') add(gy, a, b);
            else if (ch == 'S') add(gy, b, a);
            else if (ch == 'E') add(gx, a, b);
            else if (ch == 'W') add(gx, b, a);
        }
    }
    return !(hasCycle(gx) || hasCycle(gy));
}

int main() {
    vector<string> ex1 = {"A N B", "B NE C", "C N A"};
    vector<string> ex2 = {"A NW B", "A N B"};
    cout << (!validate(ex1)
        ? "does not validate, since A cannot be both north and south of C."
        : "is considered valid.") << "\n";
    cout << (validate(ex2) ? "is considered valid." : "does not validate.") << "\n";
    return 0;
}
