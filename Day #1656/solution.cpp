// Trie autocomplete: insert words, DFS from prefix node in child-insertion order
// (ordered key list). O(total chars) build, O(matches) query; O(total chars) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    unordered_map<char, Node*> children;
    vector<char> order;
    string word;
    bool isWord = false;
};

struct Trie {
    Node *root = new Node();
    void insert(const string &w) {
        Node *n = root;
        for (char c : w) {
            if (!n->children.count(c)) {
                n->children[c] = new Node();
                n->order.push_back(c);
            }
            n = n->children[c];
        }
        n->word = w;
        n->isWord = true;
    }
    void dfs(Node *n, vector<string> &out) {
        if (n->isWord) out.push_back(n->word);
        for (char c : n->order) dfs(n->children[c], out);
    }
    vector<string> autocomplete(const string &s) {
        Node *n = root;
        for (char c : s) {
            if (!n->children.count(c)) return {};
            n = n->children[c];
        }
        vector<string> out;
        dfs(n, out);
        return out;
    }
};

int main() {
    Trie t;
    for (const string &w : {"dog", "deer", "deal"}) t.insert(w);
    vector<string> res = t.autocomplete("de");
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << (i ? ", " : "") << res[i];
    cout << "]\n";
    return 0;
}
