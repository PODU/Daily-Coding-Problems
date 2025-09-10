// Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
// exists iff in-degree==out-degree for every node and edges form one connected component.
// Find the circuit with Hierholzer's algorithm. O(V + E) time and space.
#include <bits/stdc++.h>
using namespace std;

bool circleOrder(const vector<string>& words, vector<string>& order) {
    map<char, vector<pair<char, string>>> adj; // first char -> (last char, word)
    map<char, int> indeg, outdeg;
    set<char> nodes;
    for (const string& w : words) {
        char a = w.front(), b = w.back();
        adj[a].push_back({b, w});
        outdeg[a]++; indeg[b]++;
        nodes.insert(a); nodes.insert(b);
    }
    for (char c : nodes) if (indeg[c] != outdeg[c]) return false;

    // Connectivity check over nodes that have edges (treat graph as undirected).
    map<char, vector<char>> und;
    for (auto& kv : adj) for (auto& e : kv.second) { und[kv.first].push_back(e.first); und[e.first].push_back(kv.first); }
    set<char> active;
    for (auto& kv : adj) if (!kv.second.empty()) active.insert(kv.first);
    if (active.empty()) return false;
    set<char> seen;
    vector<char> st = {*active.begin()};
    while (!st.empty()) {
        char c = st.back(); st.pop_back();
        if (seen.count(c)) continue;
        seen.insert(c);
        for (char nb : und[c]) if (!seen.count(nb)) st.push_back(nb);
    }
    for (char c : active) if (!seen.count(c)) return false;

    // Hierholzer from the first word's first char so the demo starts at that word.
    char start = words.front().front();
    map<char, size_t> ptr;
    vector<char> nodeStack = {start};
    vector<string> edgeStack, circuit;
    while (!nodeStack.empty()) {
        char v = nodeStack.back();
        if (ptr[v] < adj[v].size()) {
            pair<char, string> e = adj[v][ptr[v]++];
            nodeStack.push_back(e.first);
            edgeStack.push_back(e.second);
        } else {
            nodeStack.pop_back();
            if (!edgeStack.empty()) { circuit.push_back(edgeStack.back()); edgeStack.pop_back(); }
        }
    }
    if (circuit.size() != words.size()) return false;
    reverse(circuit.begin(), circuit.end());
    order = circuit;
    return true;
}

int main() {
    vector<string> words = {"chair", "height", "racket", "touch", "tunic"};
    vector<string> order;
    if (circleOrder(words, order)) {
        for (size_t i = 0; i < order.size(); ++i) cout << order[i] << " --> ";
        cout << order.front() << "\n";
    } else {
        cout << "No circle possible\n";
    }
    return 0;
}
