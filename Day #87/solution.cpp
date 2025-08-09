// Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
// edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
#include <bits/stdc++.h>
using namespace std;

struct Graph {
    map<string, set<string>> adj;
    void edge(const string& a, const string& b) { adj[a].insert(b); }
    bool hasCycleUtil(const string& u, map<string,int>& state) {
        state[u] = 1; // visiting
        for (auto& v : adj[u]) {
            if (state[v] == 1) return true;
            if (state[v] == 0 && hasCycleUtil(v, state)) return true;
        }
        state[u] = 2; // done
        return false;
    }
    bool hasCycle() {
        map<string,int> state;
        for (auto& kv : adj)
            if (state[kv.first] == 0 && hasCycleUtil(kv.first, state)) return true;
        return false;
    }
};

bool validate(const vector<array<string,3>>& rules) {
    Graph vert, horiz; // u->v means u greater (north / east)
    for (auto& r : rules) {
        const string& a = r[0]; const string& dir = r[1]; const string& b = r[2];
        for (char c : dir) {
            if (c == 'N') vert.edge(a, b);
            else if (c == 'S') vert.edge(b, a);
            else if (c == 'E') horiz.edge(a, b);
            else if (c == 'W') horiz.edge(b, a);
        }
    }
    return !vert.hasCycle() && !horiz.hasCycle();
}

int main() {
    vector<array<string,3>> rules = {{"A","N","B"},{"B","NE","C"},{"C","N","A"}};
    cout << (validate(rules) ? "validates" : "does not validate") << "\n";
    // does not validate
    return 0;
}
