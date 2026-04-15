// Topological DP over DAG: dp[node][c] = max count of letter c on a path ending at node.
// Kahn's algorithm detects cycles (return -1 / null). Time O((V+E)*26), Space O(V*26).
#include <bits/stdc++.h>
using namespace std;

// Returns largest path value, or -1 if a cycle exists (null case).
long long largestPathValue(const string& colors, const vector<pair<int,int>>& edges) {
    int n = colors.size();
    vector<vector<int>> adj(n);
    vector<int> indeg(n, 0);
    for (auto& e : edges) {
        adj[e.first].push_back(e.second);
        indeg[e.second]++;
    }

    vector<array<int,26>> dp(n);
    for (auto& d : dp) d.fill(0);

    queue<int> q;
    for (int i = 0; i < n; i++) if (indeg[i] == 0) q.push(i);

    int processed = 0;
    long long ans = 0;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        processed++;
        dp[u][colors[u]-'A']++;
        ans = max(ans, (long long)dp[u][colors[u]-'A']);
        for (int v : adj[u]) {
            for (int c = 0; c < 26; c++)
                dp[v][c] = max(dp[v][c], dp[u][c]);
            if (--indeg[v] == 0) q.push(v);
        }
    }
    if (processed < n) return -1; // cycle -> null
    return ans;
}

int main() {
    string colors = "ABACA";
    vector<pair<int,int>> edges = {{0,1},{0,2},{2,3},{3,4}};
    long long res = largestPathValue(colors, edges);
    if (res < 0) cout << "null\n";
    else cout << res << "\n";
    return 0;
}
