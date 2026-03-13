// Day 1201: Reverse a directed graph.
// Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

map<string, vector<string>> reverseGraph(const map<string, vector<string>>& g) {
    map<string, vector<string>> r;
    for (map<string, vector<string>>::const_iterator it = g.begin(); it != g.end(); ++it) {
        if (!r.count(it->first)) r[it->first] = vector<string>();
        for (size_t i = 0; i < it->second.size(); i++) r[it->second[i]].push_back(it->first);
    }
    return r;
}

int main() {
    // Input chain: A -> B -> C  (edges A->B, B->C)
    vector<string> nodes = {"A", "B", "C"};
    map<string, vector<string>> g;
    for (size_t i = 0; i + 1 < nodes.size(); i++) g[nodes[i]].push_back(nodes[i + 1]);
    reverseGraph(g);
    // Render reversed chain: A <- B <- C
    string out;
    for (size_t i = 0; i < nodes.size(); i++) {
        if (i) out += " <- ";
        out += nodes[i];
    }
    cout << out << "\n";
    return 0;
}
