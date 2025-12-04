// Day 692: Autocomplete - return all dictionary strings having s as a prefix.
// Approach: Trie. Insert all words O(total chars). Query walks prefix then DFS to
// collect words. Query O(|s| + #matches * len). Build O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    struct Node { unordered_map<char, Node*> ch; bool end = false; };
    Node* root = new Node();
    void insert(const string& w) {
        Node* cur = root;
        for (char c : w) { if (!cur->ch.count(c)) cur->ch[c] = new Node(); cur = cur->ch[c]; }
        cur->end = true;
    }
    void dfs(Node* n, string& cur, vector<string>& out) {
        if (n->end) out.push_back(cur);
        for (auto& [c, nx] : n->ch) { cur.push_back(c); dfs(nx, cur, out); cur.pop_back(); }
    }
    vector<string> autocomplete(const string& s) {
        Node* cur = root;
        for (char c : s) { if (!cur->ch.count(c)) return {}; cur = cur->ch[c]; }
        vector<string> out; string buf = s; dfs(cur, buf, out); return out;
    }
};

int main() {
    Trie t;
    for (string w : {"dog", "deer", "deal"}) t.insert(w);
    vector<string> res = t.autocomplete("de");
    sort(res.begin(), res.end());
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";   // [deal, deer]
    return 0;
}
