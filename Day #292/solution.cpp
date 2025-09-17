// Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
// Time O(V+E), Space O(V).
#include <bits/stdc++.h>
using namespace std;

bool twoColor(map<int,vector<int>>& g, vector<int>& color, int n) {
    for (int s = 0; s < n; ++s) {
        if (color[s] != -1) continue;
        queue<int> q; q.push(s); color[s] = 0;
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : g[u]) {
                if (color[v] == -1) { color[v] = color[u] ^ 1; q.push(v); }
                else if (color[v] == color[u]) return false;
            }
        }
    }
    return true;
}

void solve(map<int,vector<int>> g, int n) {
    vector<int> color(n, -1);
    if (!twoColor(g, color, n)) { cout << "False\n"; return; }
    vector<int> a, b;
    for (int i = 0; i < n; ++i) (color[i] == 0 ? a : b).push_back(i);
    auto pr = [](vector<int>& s){
        cout << "{";
        for (size_t i = 0; i < s.size(); ++i) cout << s[i] << (i+1<s.size() ? ", " : "");
        cout << "}";
    };
    pr(a); cout << " and "; pr(b); cout << "\n";
}

int main() {
    map<int,vector<int>> g1 = {{0,{3}},{1,{2}},{2,{1,4}},{3,{0,4,5}},{4,{2,3}},{5,{3}}};
    map<int,vector<int>> g2 = {{0,{3}},{1,{2}},{2,{1,3,4}},{3,{0,2,4,5}},{4,{2,3}},{5,{3}}};
    solve(g1, 6);
    solve(g2, 6);
    return 0;
}
