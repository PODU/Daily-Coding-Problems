// Day 1203: Alien dictionary - derive letter order from sorted words.
// Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
#include <bits/stdc++.h>
using namespace std;

vector<char> alienOrder(const vector<string>& words) {
    map<char, set<char>> adj;
    map<char, int> indeg;
    for (auto& w : words) for (char c : w) indeg[c]; // ensure all letters present
    for (size_t i = 0; i + 1 < words.size(); i++) {
        const string& a = words[i]; const string& b = words[i + 1];
        size_t n = min(a.size(), b.size()), j = 0;
        for (; j < n; j++) if (a[j] != b[j]) {
            if (!adj[a[j]].count(b[j])) { adj[a[j]].insert(b[j]); indeg[b[j]]++; }
            break;
        }
    }
    queue<char> q;
    for (map<char,int>::iterator it = indeg.begin(); it != indeg.end(); ++it)
        if (it->second == 0) q.push(it->first);
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
    auto o = alienOrder(words);
    cout << "[";
    for (size_t i = 0; i < o.size(); i++) { if (i) cout << ", "; cout << "'" << o[i] << "'"; }
    cout << "]\n"; // ['x', 'z', 'w', 'y']
    return 0;
}
