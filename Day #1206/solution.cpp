// Day 1206: Validate directional (N/S/E/W) rules for consistency.
// Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

struct Graph {
    map<string, vector<string>> adj;
    map<string, int> color; // 0 unvisited,1 in-stack,2 done
    void add(const string& u, const string& v) { adj[u].push_back(v); adj[v]; }
    bool dfs(const string& u) {
        color[u] = 1;
        for (auto& v : adj[u]) {
            if (color[v] == 1) return true;       // back edge -> cycle
            if (color[v] == 0 && dfs(v)) return true;
        }
        color[u] = 2;
        return false;
    }
    bool hasCycle() {
        for (map<string, vector<string>>::iterator it = adj.begin(); it != adj.end(); ++it)
            if (color[it->first] == 0 && dfs(it->first)) return true;
        return false;
    }
};

bool validate(const vector<array<string,3>>& rules) {
    Graph gy, gx; // edge u->v means u greater (north / east) than v
    for (auto& r : rules) {
        const string& a = r[0]; const string& d = r[1]; const string& b = r[2];
        if (d.find('N') != string::npos) gy.add(a, b);
        if (d.find('S') != string::npos) gy.add(b, a);
        if (d.find('E') != string::npos) gx.add(a, b);
        if (d.find('W') != string::npos) gx.add(b, a);
    }
    return !gy.hasCycle() && !gx.hasCycle();
}

int main() {
    vector<array<string,3>> rules = {{"A","N","B"}, {"B","NE","C"}, {"C","N","A"}};
    cout << (validate(rules) ? "validates" : "does not validate") << "\n"; // does not validate
    return 0;
}
