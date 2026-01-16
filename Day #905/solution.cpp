// Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
// Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
#include <bits/stdc++.h>
using namespace std;

int indeg[26], outdeg[26];
vector<int> adj[26], radj[26];
bool vis[26];

void dfs(vector<int>* g, int u) {
    vis[u] = true;
    for (int v : g[u]) if (!vis[v]) dfs(g, v);
}

bool canChain(const vector<string>& words) {
    for (int i = 0; i < 26; i++) { indeg[i]=outdeg[i]=0; adj[i].clear(); radj[i].clear(); }
    for (auto& w : words) {
        int a = w.front()-'a', b = w.back()-'a';
        outdeg[a]++; indeg[b]++;
        adj[a].push_back(b); radj[b].push_back(a);
    }
    for (int i = 0; i < 26; i++) if (indeg[i] != outdeg[i]) return false;
    int start = -1;
    for (int i = 0; i < 26; i++) if (outdeg[i] > 0) { start = i; break; }
    if (start == -1) return true; // no words
    memset(vis, 0, sizeof(vis));
    dfs(adj, start);
    for (int i = 0; i < 26; i++) if ((indeg[i]||outdeg[i]) && !vis[i]) return false;
    memset(vis, 0, sizeof(vis));
    dfs(radj, start);
    for (int i = 0; i < 26; i++) if ((indeg[i]||outdeg[i]) && !vis[i]) return false;
    return true;
}

int main() {
    vector<string> words = {"chair","height","racket","touch","tunic"};
    cout << (canChain(words) ? "True" : "False") << "\n";
    return 0;
}
