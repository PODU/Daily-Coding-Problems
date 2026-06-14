// DFS subtree sizes; count non-root subtrees with even size = max edges removable.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;
vector<vector<int>> g;
int ans = 0;
int dfs(int u, int p) {
    int sz = 1;
    for (int v : g[u]) if (v != p) sz += dfs(v, u);
    if (p != -1 && sz % 2 == 0) ans++;
    return sz;
}
int main() {
    int n = 8;
    g.assign(n + 1, {});
    auto add = [&](int a, int b){ g[a].push_back(b); g[b].push_back(a); };
    add(1,2); add(1,3); add(3,4); add(3,5); add(4,6); add(4,7); add(4,8);
    dfs(1, -1);
    cout << ans << "\n";
    return 0;
}
