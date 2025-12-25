// Day 799: PrefixMapSum - trie where each node stores sum of values below it.
// insert overwrites via delta (new-old) propagated along the path.
// insert O(L), sum O(L). Space O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    long long total = 0;
    Node* child[26] = {};
};

struct PrefixMapSum {
    Node* root = new Node();
    unordered_map<string, int> vals;

    void insert(const string& key, int value) {
        int delta = value - (vals.count(key) ? vals[key] : 0);
        vals[key] = value;
        Node* cur = root;
        cur->total += delta;
        for (char c : key) {
            int i = c - 'a';
            if (!cur->child[i]) cur->child[i] = new Node();
            cur = cur->child[i];
            cur->total += delta;
        }
    }

    long long sum(const string& prefix) {
        Node* cur = root;
        for (char c : prefix) {
            int i = c - 'a';
            if (!cur->child[i]) return 0;
            cur = cur->child[i];
        }
        return cur->total;
    }
};

int main() {
    PrefixMapSum m;
    m.insert("columnar", 3);
    cout << m.sum("col") << "\n"; // 3
    m.insert("column", 2);
    cout << m.sum("col") << "\n"; // 5
    return 0;
}
