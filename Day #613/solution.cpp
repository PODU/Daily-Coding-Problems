// Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
// Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).
#include <bits/stdc++.h>
using namespace std;

class PrefixMapSum {
    struct Node {
        long long sum = 0;
        unordered_map<char, Node*> ch;
    };
    Node* root = new Node();
    unordered_map<string, long long> values;
public:
    void insert(const string& key, long long value) {
        long long delta = value - values[key];
        values[key] = value;
        Node* node = root;
        for (char c : key) {
            if (!node->ch.count(c)) node->ch[c] = new Node();
            node = node->ch[c];
            node->sum += delta;
        }
    }
    long long sum(const string& prefix) {
        Node* node = root;
        for (char c : prefix) {
            if (!node->ch.count(c)) return 0;
            node = node->ch[c];
        }
        return node->sum;
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
