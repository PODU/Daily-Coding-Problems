// Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<vector<int>> graph = {{0,1,3},{1,2},{2},{3}};
    int n = graph.size();
    vector<vector<int>> M(n, vector<int>(n, 0));
    for (int s = 0; s < n; s++) {
        vector<int> vis(n, 0);
        stack<int> st; st.push(s);
        while (!st.empty()) {
            int u = st.top(); st.pop();
            if (vis[u]) continue;
            vis[u] = 1; M[s][u] = 1;
            for (int v : graph[u]) if (!vis[v]) st.push(v);
        }
    }
    for (int i = 0; i < n; i++) {
        cout << "[";
        for (int j = 0; j < n; j++) {
            cout << M[i][j];
            if (j + 1 < n) cout << ", ";
        }
        cout << "]\n";
    }
    return 0;
}
