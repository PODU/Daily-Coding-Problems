// Day 728: Split students into two teams so no enemies share a team.
// Approach: BFS 2-coloring (bipartite check). Returns two teams or reports failure.
// Time: O(V + E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

// returns {true, colors} if bipartite
pair<bool, vector<int>> twoColor(map<int, vector<int>>& g) {
    int n = g.size();
    vector<int> color(n, -1);
    for (auto& kv : g) {
        int s = kv.first;
        if (color[s] != -1) continue;
        color[s] = 0;
        queue<int> q; q.push(s);
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : g[u]) {
                if (color[v] == -1) { color[v] = color[u] ^ 1; q.push(v); }
                else if (color[v] == color[u]) return {false, {}};
            }
        }
    }
    return {true, color};
}

string setStr(const vector<int>& v) {
    string s = "{";
    for (size_t i = 0; i < v.size(); i++) s += to_string(v[i]) + (i + 1 < v.size() ? ", " : "");
    return s + "}";
}

void solve(map<int, vector<int>> g) {
    auto result = twoColor(g);
    if (!result.first) { cout << "False\n"; return; }
    vector<int>& color = result.second;
    vector<int> a, b;
    for (auto& kv : g) (color[kv.first] == 0 ? a : b).push_back(kv.first);
    cout << setStr(a) << " and " << setStr(b) << "\n";
}

int main() {
    solve({{0,{3}},{1,{2}},{2,{1,4}},{3,{0,4,5}},{4,{2,3}},{5,{3}}});
    solve({{0,{3}},{1,{2}},{2,{1,3,4}},{3,{0,2,4,5}},{4,{2,3}},{5,{3}}});
    return 0;
}
