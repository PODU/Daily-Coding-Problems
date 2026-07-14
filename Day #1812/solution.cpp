// Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
// BFS coloring over each component. Time: O(V+E). Space: O(V).
#include <bits/stdc++.h>
using namespace std;

bool divideTeams(map<int, vector<int>>& g, set<int>& teamA, set<int>& teamB) {
    map<int,int> color;
    for (auto& kv : g) {
        int s = kv.first;
        if (color.count(s)) continue;
        queue<int> q; q.push(s); color[s] = 0;
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : g[u]) {
                if (!color.count(v)) { color[v] = color[u] ^ 1; q.push(v); }
                else if (color[v] == color[u]) return false;
            }
        }
    }
    for (auto& kv : color) (kv.second == 0 ? teamA : teamB).insert(kv.first);
    return true;
}

void printSet(const set<int>& s) {
    cout << "{";
    bool first = true;
    for (int x : s) { if (!first) cout << ", "; cout << x; first = false; }
    cout << "}";
}

int main() {
    map<int, vector<int>> g = {
        {0,{3}},{1,{2}},{2,{1,4}},{3,{0,4,5}},{4,{2,3}},{5,{3}}
    };
    set<int> A, B;
    if (divideTeams(g, A, B)) { printSet(A); cout << " and "; printSet(B); cout << "\n"; }
    else cout << "False\n";
    // {0, 1, 4, 5} and {2, 3}
    return 0;
}
