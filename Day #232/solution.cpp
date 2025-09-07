// PrefixMapSum: Trie where each node stores the running sum of values passing through it.
// On overwrite, propagate delta = new - old. insert/sum both O(key length).
#include <bits/stdc++.h>
using namespace std;

struct PrefixMapSum {
    struct Node { map<char, Node*> next; int sum = 0; };
    Node* root = new Node();
    unordered_map<string, int> vals;

    void insert(const string& key, int value) {
        int delta = value - (vals.count(key) ? vals[key] : 0);
        vals[key] = value;
        Node* n = root;
        for (char c : key) {
            if (!n->next.count(c)) n->next[c] = new Node();
            n = n->next[c];
            n->sum += delta;
        }
    }

    int sum(const string& prefix) {
        Node* n = root;
        for (char c : prefix) {
            if (!n->next.count(c)) return 0;
            n = n->next[c];
        }
        return n->sum;
    }
};

int main() {
    PrefixMapSum mapsum;
    mapsum.insert("columnar", 3);
    cout << mapsum.sum("col") << "\n"; // 3
    mapsum.insert("column", 2);
    cout << mapsum.sum("col") << "\n"; // 5
}
