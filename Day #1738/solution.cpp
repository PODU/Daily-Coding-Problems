// Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
// Kahn's algorithm; cycle => null. Time O((n+m)*26), Space O(n*26).
#include <bits/stdc++.h>
using namespace std;

string largestPathValue(const string& s, const vector<pair<int,int>>& edges) {
    int n = s.size();
    vector<vector<int>> adj(n);
    vector<int> indeg(n, 0);
    for (auto& e : edges) { adj[e.first].push_back(e.second); indeg[e.second]++; }
    vector<array<int,26>> dp(n);
    for (auto& a : dp) a.fill(0);
    queue<int> q;
    for (int i = 0; i < n; i++) if (indeg[i] == 0) q.push(i);
    int seen = 0, ans = 0;
    while (!q.empty()) {
        int u = q.front(); q.pop(); seen++;
        dp[u][s[u]-'A']++;
        ans = max(ans, dp[u][s[u]-'A']);
        for (int v : adj[u]) {
            for (int c = 0; c < 26; c++) dp[v][c] = max(dp[v][c], dp[u][c]);
            if (--indeg[v] == 0) q.push(v);
        }
    }
    if (seen < n) return "null";
    return to_string(ans);
}

int main() {
    cout << largestPathValue("ABACA", {{0,1},{0,2},{2,3},{3,4}}) << "\n";
    cout << largestPathValue("A", {{0,0}}) << "\n";
    return 0;
}
