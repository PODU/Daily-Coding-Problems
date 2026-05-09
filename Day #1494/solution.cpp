// Day 1494: Reverse a directed graph by reversing every edge.
// Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Input graph: A -> B -> C
    vector<pair<string,string>> edges = {{"A","B"},{"B","C"}};

    // Build reversed adjacency list.
    map<string, vector<string>> rev;
    set<string> nodes;
    for (auto& e : edges) {
        rev[e.second].push_back(e.first); // v -> u
        nodes.insert(e.first);
        nodes.insert(e.second);
    }

    // Show original as A -> B -> C and reversed as A <- B <- C
    cout << "Original: A -> B -> C\n";
    cout << "Reversed: A <- B <- C\n";

    cout << "Reversed edges:\n";
    for (auto& e : edges)
        cout << "  " << e.second << " -> " << e.first << "\n";
    return 0;
}
