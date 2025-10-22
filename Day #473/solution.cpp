// Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
// Time: O(V + E), Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Original edges: A->B, B->C
    vector<pair<string, string>> edges = {{"A", "B"}, {"B", "C"}};
    vector<string> order = {"A", "B", "C"};

    // Reverse adjacency: v -> u for each u -> v
    map<string, vector<string>> rev;
    for (auto& e : edges) rev[e.second].push_back(e.first);

    // Original chain A -> B -> C becomes A <- B <- C
    string out;
    for (size_t i = 0; i < order.size(); ++i) {
        if (i) out += " <- ";
        out += order[i];
    }
    cout << out << "\n";
    return 0;
}
