// Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
// Time: O(V+E), Space: O(V+E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    auto solve = [](map<int, vector<int>> students) -> pair<bool, array<vector<int>, 2>> {
        map<int, int> color;
        array<vector<int>, 2> teams;
        for (auto &kv : students) {
            int start = kv.first;
            if (color.count(start)) continue;
            queue<int> q;
            q.push(start);
            color[start] = 0;
            while (!q.empty()) {
                int u = q.front(); q.pop();
                vector<int> nbrs = students[u];
                sort(nbrs.begin(), nbrs.end());
                for (int v : nbrs) {
                    if (!color.count(v)) {
                        color[v] = color[u] ^ 1;
                        q.push(v);
                    } else if (color[v] == color[u]) {
                        return {false, teams};
                    }
                }
            }
        }
        for (auto &kv : color) teams[kv.second].push_back(kv.first);
        for (auto &t : teams) sort(t.begin(), t.end());
        return {true, teams};
    };

    auto printGroup = [](const vector<int> &g) {
        cout << "{";
        for (size_t i = 0; i < g.size(); ++i) {
            if (i) cout << ", ";
            cout << g[i];
        }
        cout << "}";
    };

    map<int, vector<int>> s1 = {{0,{3}},{1,{2}},{2,{1,4}},{3,{0,4,5}},{4,{2,3}},{5,{3}}};
    auto r1 = solve(s1);
    if (r1.first) {
        printGroup(r1.second[0]);
        cout << " and ";
        printGroup(r1.second[1]);
        cout << "\n";
    } else {
        cout << "False\n";
    }

    map<int, vector<int>> s2 = {{0,{3}},{1,{2}},{2,{1,3,4}},{3,{0,2,4,5}},{4,{2,3}},{5,{3}}};
    auto r2 = solve(s2);
    if (r2.first) {
        printGroup(r2.second[0]);
        cout << " and ";
        printGroup(r2.second[1]);
        cout << "\n";
    } else {
        cout << "False\n";
    }
    return 0;
}
