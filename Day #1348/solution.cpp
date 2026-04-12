// Weighted tree diameter: DFS, at each node track two largest downward child path sums;
// diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

vector<vector<pair<int,int>>> adj;
int best = 0;

int dfs(int u, int parent) {
    int max1 = 0, max2 = 0; // two largest downward path sums
    for (auto& e : adj[u]) {
        int v = e.first, w = e.second;
        if (v == parent) continue;
        int down = dfs(v, u) + w;
        if (down > max1) { max2 = max1; max1 = down; }
        else if (down > max2) { max2 = down; }
    }
    best = max(best, max1 + max2);
    return max1;
}

int main() {
    int n = 8; // a..h -> 0..7
    adj.assign(n, {});
    auto add = [&](int a, int b, int w){ adj[a].push_back({b,w}); adj[b].push_back({a,w}); };
    add(0,1,3); // a-b
    add(0,2,5); // a-c
    add(0,3,8); // a-d
    add(3,4,2); // d-e
    add(3,5,4); // d-f
    add(4,6,1); // e-g
    add(4,7,1); // e-h
    dfs(0, -1);
    cout << best << "\n";
    return 0;
}
