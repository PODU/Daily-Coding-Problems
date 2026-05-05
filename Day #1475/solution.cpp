// Day 1475: Autocomplete via trie. Walk to prefix node, collect subtree words.
// Build O(total chars); query O(len(prefix) + matches). Space O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    map<char, Node*> next;
    int order = -1;
    string word;
};

struct Trie {
    Node* root = new Node();
    void insert(const string& w, int order) {
        Node* node = root;
        for (char ch : w) {
            if (!node->next.count(ch)) node->next[ch] = new Node();
            node = node->next[ch];
        }
        node->order = order;
        node->word = w;
    }
    void collect(Node* n, vector<pair<int, string>>& out) {
        if (n->order >= 0) out.push_back({n->order, n->word});
        for (auto& kv : n->next) collect(kv.second, out);
    }
    vector<string> startsWith(const string& prefix) {
        Node* node = root;
        for (char ch : prefix) {
            if (!node->next.count(ch)) return {};
            node = node->next[ch];
        }
        vector<pair<int, string>> out;
        collect(node, out);
        sort(out.begin(), out.end());
        vector<string> res;
        for (auto& p : out) res.push_back(p.second);
        return res;
    }
};

int main() {
    Trie t;
    vector<string> dict = {"dog", "deer", "deal"};
    for (int i = 0; i < (int)dict.size(); ++i) t.insert(dict[i], i);
    auto res = t.startsWith("de");
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) cout << (i ? ", " : "") << res[i];
    cout << "]\n";  // [deer, deal]
}
