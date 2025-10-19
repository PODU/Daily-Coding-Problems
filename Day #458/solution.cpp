// Day 458: Validate directional (NE/SW/...) rules for consistency.
// Decompose into x/y strict orders; a cycle in either graph = contradiction.
// Time O(R + V) via DFS cycle detection.
#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <sstream>
using namespace std;

struct Graph {
    map<string, vector<string>> adj;
    map<string, int> color; // 0 unvisited,1 in-stack,2 done
    void addLess(const string& small, const string& big) { adj[small].push_back(big); color[small]; color[big]; }
    bool dfs(const string& u) {
        color[u] = 1;
        for (auto& v : adj[u]) {
            if (color[v] == 1) return true;          // back edge -> cycle
            if (color[v] == 0 && dfs(v)) return true;
        }
        color[u] = 2;
        return false;
    }
    bool hasCycle() {
        for (auto& kv : color) if (color[kv.first] == 0 && dfs(kv.first)) return true;
        return false;
    }
};

bool validate(const vector<string>& rules) {
    Graph gx, gy; // edge small->big means small < big
    for (auto& r : rules) {
        istringstream iss(r);
        string a, d, b;
        iss >> a >> d >> b;
        for (char c : d) {
            if (c == 'N') gy.addLess(b, a);      // a north of b: a.y > b.y
            else if (c == 'S') gy.addLess(a, b);
            else if (c == 'E') gx.addLess(b, a); // a east of b: a.x > b.x
            else if (c == 'W') gx.addLess(a, b);
        }
    }
    return !gx.hasCycle() && !gy.hasCycle();
}

int main() {
    vector<string> rules = {"A N B", "B NE C", "C N A"};
    cout << (validate(rules) ? "Valid" : "Invalid") << endl; // Invalid
    cout << (validate({"A NW B", "A N B"}) ? "Valid" : "Invalid") << endl; // Valid
    return 0;
}
