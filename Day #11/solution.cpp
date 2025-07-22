// Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
// Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
#include <bits/stdc++.h>
using namespace std;

struct TrieNode {
    map<char, TrieNode*> ch;
    int order = -1; // insertion index if a word ends here
};

struct Trie {
    TrieNode* root = new TrieNode();
    int counter = 0;

    void insert(const string& w) {
        TrieNode* cur = root;
        for (char c : w) {
            if (!cur->ch.count(c)) cur->ch[c] = new TrieNode();
            cur = cur->ch[c];
        }
        cur->order = counter++;
    }

    vector<string> autocomplete(const string& prefix) {
        TrieNode* cur = root;
        for (char c : prefix) {
            if (!cur->ch.count(c)) return {};
            cur = cur->ch[c];
        }
        vector<pair<int, string>> found;
        string buf = prefix;
        dfs(cur, buf, found);
        sort(found.begin(), found.end());
        vector<string> res;
        for (auto& p : found) res.push_back(p.second);
        return res;
    }

    void dfs(TrieNode* n, string& buf, vector<pair<int, string>>& found) {
        if (n->order >= 0) found.push_back({n->order, buf});
        for (auto& kv : n->ch) {
            buf.push_back(kv.first);
            dfs(kv.second, buf, found);
            buf.pop_back();
        }
    }
};

int main() {
    Trie t;
    for (string w : {"dog", "deer", "deal"}) t.insert(w);
    auto res = t.autocomplete("de");
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
