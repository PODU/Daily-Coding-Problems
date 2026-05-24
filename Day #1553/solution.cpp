// Alien dictionary: build edges from first differing chars of adjacent words,
// then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
#include <bits/stdc++.h>
using namespace std;

vector<char> alienOrder(const vector<string>& words) {
    map<char, set<char>> adj;
    map<char, int> indeg;
    for (const auto& w : words)
        for (char c : w) indeg[c]; // ensure present
    for (size_t i = 0; i + 1 < words.size(); ++i) {
        const string& a = words[i];
        const string& b = words[i + 1];
        size_t n = min(a.size(), b.size());
        size_t j = 0;
        for (; j < n; ++j) {
            if (a[j] != b[j]) {
                if (!adj[a[j]].count(b[j])) {
                    adj[a[j]].insert(b[j]);
                    indeg[b[j]]++;
                }
                break;
            }
        }
        // invalid prefix case: longer prefix before shorter word
        if (j == n && a.size() > b.size()) return {};
    }
    priority_queue<char, vector<char>, greater<char>> pq;
    for (auto& p : indeg) if (p.second == 0) pq.push(p.first);
    vector<char> order;
    while (!pq.empty()) {
        char c = pq.top(); pq.pop();
        order.push_back(c);
        for (char nx : adj[c]) if (--indeg[nx] == 0) pq.push(nx);
    }
    if (order.size() != indeg.size()) return {};
    return order;
}

int main() {
    vector<string> words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
    vector<char> order = alienOrder(words);
    cout << "[";
    for (size_t i = 0; i < order.size(); ++i) {
        cout << "'" << order[i] << "'";
        if (i + 1 < order.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
