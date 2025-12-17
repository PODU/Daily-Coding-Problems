// Day 755: Largest value path in a directed graph.
// Topological DP: dp[u][c] = max count of letter c on a path starting at u.
// Cycle -> value is infinite -> null. Time: O((n+m)*26), Space: O(n*26).
#include <bits/stdc++.h>
using namespace std;

// returns {hasAnswer, value}
pair<bool,int> largestPathValue(const string& letters,
                                const vector<pair<int,int>>& edges) {
    int n = letters.size();
    vector<vector<int>> adj(n);
    vector<int> indeg(n, 0);
    for (auto& e : edges) { adj[e.first].push_back(e.second); indeg[e.second]++; }

    // Kahn topological sort
    queue<int> q;
    for (int i = 0; i < n; ++i) if (indeg[i] == 0) q.push(i);
    vector<int> topo;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        topo.push_back(u);
        for (int v : adj[u]) if (--indeg[v] == 0) q.push(v);
    }
    if ((int)topo.size() < n) return {false, 0};   // cycle -> null

    vector<array<int,26>> dp(n);
    for (int i = 0; i < n; ++i) { dp[i].fill(0); dp[i][letters[i]-'A'] = 1; }

    int best = 0;
    for (int i = (int)topo.size() - 1; i >= 0; --i) {
        int u = topo[i];
        for (int v : adj[u])
            for (int c = 0; c < 26; ++c) {
                int add = dp[v][c] + (c == letters[u]-'A' ? 1 : 0);
                if (add > dp[u][c]) dp[u][c] = add;
            }
        for (int c = 0; c < 26; ++c) best = max(best, dp[u][c]);
    }
    return {true, best};
}

int main() {
    string letters = "ABACA";
    vector<pair<int,int>> edges = {{0,1},{0,2},{2,3},{3,4}};
    auto res = largestPathValue(letters, edges);
    if (res.first) cout << res.second << "\n"; else cout << "null\n";  // 3
    return 0;
}
