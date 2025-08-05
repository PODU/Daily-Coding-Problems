// Largest path value in directed graph: topo sort (Kahn) + DP dp[node][26]. Cycle -> null. Time O((n+m)*26), Space O(n*26).
#include <bits/stdc++.h>
using namespace std;

// Graph "A" with edge (0,0) returns null (self-loop is a cycle).
// Returns true and sets result if finite; returns false if cyclic (null).
bool largestPathValue(const string& colors, const vector<pair<int,int>>& edges, int& result) {
    int n = colors.size();
    vector<vector<int>> adj(n);
    vector<int> indeg(n, 0);
    for (auto& e : edges) {
        adj[e.first].push_back(e.second);
        indeg[e.second]++;
    }
    vector<array<int,26>> dp(n);
    for (auto& a : dp) a.fill(0);
    queue<int> q;
    for (int i = 0; i < n; i++) if (indeg[i] == 0) q.push(i);
    int seen = 0, ans = 0;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        seen++;
        dp[u][colors[u]-'A']++;
        ans = max(ans, dp[u][colors[u]-'A']);
        for (int v : adj[u]) {
            for (int c = 0; c < 26; c++)
                dp[v][c] = max(dp[v][c], dp[u][c]);
            if (--indeg[v] == 0) q.push(v);
        }
    }
    if (seen < n) return false; // cycle
    result = ans;
    return true;
}

int main() {
    string colors = "ABACA";
    vector<pair<int,int>> edges = {{0,1},{0,2},{2,3},{3,4}};
    int result;
    if (largestPathValue(colors, edges, result))
        cout << result << endl;
    else
        cout << "null" << endl;
    return 0;
}
