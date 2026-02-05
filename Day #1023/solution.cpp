// Day 1023: Alien dictionary - order of letters from sorted words.
// Approach: build precedence graph from adjacent words, Kahn's topological sort.
// Time O(total chars + V + E), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

vector<char> alienOrder(const vector<string>& words) {
    vector<char> order;                  // letters in first-appearance order
    map<char, set<char>> adj;
    map<char, int> indeg;
    auto seen = [&](char c) {
        if (!indeg.count(c)) { indeg[c] = 0; order.push_back(c); adj[c]; }
    };
    for (auto& w : words) for (char c : w) seen(c);

    for (size_t i = 0; i + 1 < words.size(); ++i) {
        const string& a = words[i];
        const string& b = words[i + 1];
        size_t n = min(a.size(), b.size()), j = 0;
        while (j < n && a[j] == b[j]) ++j;
        if (j < n) {
            if (!adj[a[j]].count(b[j])) { adj[a[j]].insert(b[j]); indeg[b[j]]++; }
        }
    }

    deque<char> q;
    for (char c : order) if (indeg[c] == 0) q.push_back(c);   // first-appearance order
    vector<char> res;
    while (!q.empty()) {
        char c = q.front(); q.pop_front();
        res.push_back(c);
        for (char nb : adj[c]) if (--indeg[nb] == 0) q.push_back(nb);
    }
    return res;
}

int main() {
    vector<string> words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
    auto res = alienOrder(words);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "'" << res[i] << "'";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
}
