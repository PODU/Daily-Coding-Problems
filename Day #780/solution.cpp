// Day 780: Topological sort of courses (prereqs map). DFS post-order with
// cycle detection; returns empty (null) if a cycle exists. O(V + E).
#include <bits/stdc++.h>
using namespace std;

bool dfs(const string& c, map<string, vector<string>>& g,
         map<string, int>& state, vector<string>& order) {
    state[c] = 1;                            // visiting
    for (auto& pre : g[c]) {
        if (state[pre] == 1) return false;   // cycle
        if (state[pre] == 0 && !dfs(pre, g, state, order)) return false;
    }
    state[c] = 2;                            // done
    order.push_back(c);
    return true;
}

vector<string> courseOrder(map<string, vector<string>> g) {
    map<string, int> state;
    vector<string> order;
    for (auto& kv : g)
        if (state[kv.first] == 0 && !dfs(kv.first, g, state, order)) return {}; // null
    return order;
}

int main() {
    map<string, vector<string>> g = {
        {"CSC300", {"CSC100", "CSC200"}},
        {"CSC200", {"CSC100"}},
        {"CSC100", {}}
    };
    auto r = courseOrder(g);
    if (r.empty()) { cout << "null\n"; return 0; }
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << "'" << r[i] << "'" << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // ['CSC100', 'CSC200', 'CSC300']
    return 0;
}
