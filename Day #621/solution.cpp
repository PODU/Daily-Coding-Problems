// Tree diameter with edge weights: DFS returning max downward path; global best
// = sum of two largest child paths. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Edge { int to; long long w; };
vector<vector<Edge>> adj;
long long best = 0;

long long dfs(int u, int parent) {
    long long max1 = 0, max2 = 0; // two largest downward path sums
    for (auto& e : adj[u]) {
        if (e.to == parent) continue;
        long long path = dfs(e.to, u) + e.w;
        if (path > max1) { max2 = max1; max1 = path; }
        else if (path > max2) { max2 = path; }
    }
    best = max(best, max1 + max2);
    return max1;
}

int main() {
    // nodes: a0 b1 c2 d3 e4 f5 g6 h7
    adj.assign(8, {});
    auto add = [&](int u, int v, long long w) {
        adj[u].push_back({v, w}); adj[v].push_back({u, w});
    };
    add(0,1,3); add(0,2,5); add(0,3,8);
    add(3,4,2); add(3,5,4);
    add(4,6,1); add(4,7,1);
    dfs(0, -1);
    cout << best << "\n";
    return 0;
}
