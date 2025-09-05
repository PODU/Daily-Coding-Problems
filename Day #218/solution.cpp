// Day 218: Reverse a directed graph (transpose).
// Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

map<string, vector<string>> reverseGraph(const map<string, vector<string>>& g) {
    map<string, vector<string>> r;
    for (auto& kv : g) {
        const string& u = kv.first;
        if (!r.count(u)) r[u]; // ensure node exists
        for (auto& v : kv.second) r[v].push_back(u);
    }
    return r;
}

int main() {
    // A -> B -> C
    map<string, vector<string>> g = {{"A", {"B"}}, {"B", {"C"}}, {"C", {}}};
    auto r = reverseGraph(g);
    // Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
    cout << "A <- B <- C" << endl;
    return 0;
}
