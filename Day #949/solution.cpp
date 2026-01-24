// Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
// Build O(total chars); query O(|s| + matches). Insertion order preserved.
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    struct Node {
        unordered_map<char, Node*> next;
        vector<int> ids; // ids of words passing through (in insertion order)
    };
    Node* root = new Node();
    void insert(const string& w, int id) {
        Node* cur = root;
        for (char c : w) {
            if (!cur->next.count(c)) cur->next[c] = new Node();
            cur = cur->next[c];
            cur->ids.push_back(id);
        }
    }
    vector<int> withPrefix(const string& s) {
        Node* cur = root;
        for (char c : s) {
            if (!cur->next.count(c)) return {};
            cur = cur->next[c];
        }
        return cur->ids;
    }
};

int main() {
    vector<string> words = {"dog", "deer", "deal"};
    Trie t;
    for (int i = 0; i < (int)words.size(); ++i) t.insert(words[i], i);
    string s = "de";
    vector<int> res = t.withPrefix(s);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << words[res[i]] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n"; // [deer, deal]
    return 0;
}
