// Direction-rule consistency: decompose each rule into strict x/y "greater-than"
// edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
// Time: O(V+E) per axis, Space: O(V+E).
#include <bits/stdc++.h>
using namespace std;

struct Checker {
    map<string, vector<string>> gx, gy; // u->v means u > v on that axis
    set<string> nodes;

    void addEdge(map<string, vector<string>>& g, const string& a, const string& b) {
        g[a].push_back(b);
        nodes.insert(a);
        nodes.insert(b);
    }
    // rule: A located <dir> of B
    void addRule(const string& a, const string& dir, const string& b) {
        for (char c : dir) {
            if (c == 'N') addEdge(gy, a, b); // a.y > b.y
            if (c == 'S') addEdge(gy, b, a); // a.y < b.y
            if (c == 'E') addEdge(gx, a, b); // a.x > b.x
            if (c == 'W') addEdge(gx, b, a); // a.x < b.x
        }
    }
    bool dfs(map<string, vector<string>>& g, const string& u,
             map<string, int>& state) {
        state[u] = 1;
        for (auto& v : g[u]) {
            if (state[v] == 1) return true;
            if (state[v] == 0 && dfs(g, v, state)) return true;
        }
        state[u] = 2;
        return false;
    }
    bool hasCycle(map<string, vector<string>>& g) {
        map<string, int> state;
        for (auto& n : nodes)
            if (state[n] == 0 && dfs(g, n, state)) return true;
        return false;
    }
    bool validates() { return !hasCycle(gx) && !hasCycle(gy); }
};

int main() {
    Checker c1;
    c1.addRule("A", "N", "B");
    c1.addRule("B", "NE", "C");
    c1.addRule("C", "N", "A");
    if (!c1.validates())
        cout << "does not validate, since A cannot be both north and south of C.\n";

    Checker c2;
    c2.addRule("A", "NW", "B");
    c2.addRule("A", "N", "B");
    if (c2.validates())
        cout << "A NW B / A N B is considered valid.\n";
    return 0;
}
