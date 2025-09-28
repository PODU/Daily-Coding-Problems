// Max edges removed so every component has even node count. DFS subtree sizes;
// answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
// Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
// cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
#include <iostream>
#include <vector>
using namespace std;

vector<vector<int>> adj;
vector<int> sz;
int answer = 0;

int dfs(int u, int parent, int root) {
    int s = 1;
    for (int v : adj[u])
        if (v != parent) s += dfs(v, u, root);
    sz[u] = s;
    if (u != root && s % 2 == 0) answer++;
    return s;
}

int main() {
    int n = 8;
    adj.assign(n + 1, {});
    sz.assign(n + 1, 0);
    auto add = [&](int a, int b) { adj[a].push_back(b); adj[b].push_back(a); };
    add(1, 2); add(1, 3);
    add(3, 4); add(3, 5);
    add(4, 6); add(4, 7); add(4, 8);
    dfs(1, 0, 1);
    cout << answer << endl;
    return 0;
}
