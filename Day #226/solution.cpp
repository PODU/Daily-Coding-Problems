// Alien Dictionary: build precedence graph from adjacent words, topological sort (Kahn's BFS).
// Time: O(total characters), Space: O(unique letters + edges).
#include <bits/stdc++.h>
using namespace std;

vector<char> alienOrder(const vector<string>& words) {
    map<char, set<char>> adj;
    map<char, int> indeg;
    for (const auto& w : words)
        for (char c : w) { adj[c]; indeg.emplace(c, 0); }
    for (size_t i = 0; i + 1 < words.size(); ++i) {
        const string &a = words[i], &b = words[i + 1];
        size_t n = min(a.size(), b.size());
        for (size_t j = 0; j < n; ++j) {
            if (a[j] != b[j]) {
                if (!adj[a[j]].count(b[j])) { adj[a[j]].insert(b[j]); indeg[b[j]]++; }
                break;
            }
        }
    }
    queue<char> q;
    for (auto& p : indeg) if (p.second == 0) q.push(p.first);
    vector<char> order;
    while (!q.empty()) {
        char c = q.front(); q.pop();
        order.push_back(c);
        for (char nx : adj[c]) if (--indeg[nx] == 0) q.push(nx);
    }
    return order;
}

int main() {
    vector<string> words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
    auto order = alienOrder(words);
    cout << "[";
    for (size_t i = 0; i < order.size(); ++i) {
        cout << "'" << order[i] << "'";
        if (i + 1 < order.size()) cout << ", ";
    }
    cout << "]\n";
}
