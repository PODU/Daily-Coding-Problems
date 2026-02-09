// Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
// Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

#include <bits/stdc++.h>
using namespace std;

// Returns {ok, teamA, teamB}
struct Result { bool ok; vector<int> a, b; };

Result divideTeams(const map<int, vector<int>>& students) {
    map<int, int> color;
    for (auto& kv : students) {
        int start = kv.first;
        if (color.count(start)) continue;
        color[start] = 0;
        queue<int> q; q.push(start);
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : students.at(u)) {
                if (!color.count(v)) { color[v] = color[u] ^ 1; q.push(v); }
                else if (color[v] == color[u]) return {false, {}, {}};
            }
        }
    }
    vector<int> a, b;
    for (auto& kv : students) (color[kv.first] == 0 ? a : b).push_back(kv.first);
    sort(a.begin(), a.end()); sort(b.begin(), b.end());
    return {true, a, b};
}

string fmt(const Result& r) {
    if (!r.ok) return "False";
    auto setStr = [](const vector<int>& v) {
        string s = "{";
        for (size_t i = 0; i < v.size(); i++) s += (i ? ", " : "") + to_string(v[i]);
        return s + "}";
    };
    return setStr(r.a) + " and " + setStr(r.b);
}

int main() {
    map<int, vector<int>> s1 = {{0,{3}},{1,{2}},{2,{1,4}},{3,{0,4,5}},{4,{2,3}},{5,{3}}};
    map<int, vector<int>> s2 = {{0,{3}},{1,{2}},{2,{1,3,4}},{3,{0,2,4,5}},{4,{2,3}},{5,{3}}};
    cout << fmt(divideTeams(s1)) << "\n";
    cout << fmt(divideTeams(s2)) << "\n";
    return 0;
}
