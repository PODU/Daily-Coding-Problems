// Day 1676: PrefixMapSum via trie storing cumulative values + delta on overwrite.
// insert/sum both O(key length). Space O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node { long total = 0; unordered_map<char, Node*> next; };

struct PrefixMapSum {
    Node* root = new Node();
    unordered_map<string, long> vals;

    void insert(const string& key, long value) {
        long delta = value - (vals.count(key) ? vals[key] : 0);
        vals[key] = value;
        Node* cur = root;
        for (char c : key) {
            if (!cur->next.count(c)) cur->next[c] = new Node();
            cur = cur->next[c];
            cur->total += delta;
        }
    }
    long sum(const string& prefix) {
        Node* cur = root;
        for (char c : prefix) {
            if (!cur->next.count(c)) return 0;
            cur = cur->next[c];
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
