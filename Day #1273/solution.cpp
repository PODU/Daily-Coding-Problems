// Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
// Trie storing accumulated sums; insert applies the delta vs the old value.
// insert/sum are O(key length).
#include <bits/stdc++.h>
using namespace std;

struct PrefixMapSum {
    struct TrieNode { long long sum = 0; unordered_map<char, TrieNode*> next; };
    TrieNode* root = new TrieNode();
    unordered_map<string, long long> vals;

    void insert(const string& key, long long value) {
        long long delta = value - (vals.count(key) ? vals[key] : 0);
        vals[key] = value;
        TrieNode* node = root;
        for (char c : key) {
            if (!node->next.count(c)) node->next[c] = new TrieNode();
            node = node->next[c];
            node->sum += delta;
        }
    }

    long long sum(const string& prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            if (!node->next.count(c)) return 0;
            node = node->next[c];
        }
        return node->sum;
    }
};

int main() {
    PrefixMapSum mapsum;
    mapsum.insert("columnar", 3);
    cout << mapsum.sum("col") << endl; // 3
    mapsum.insert("column", 2);
    cout << mapsum.sum("col") << endl; // 5
    return 0;
}
