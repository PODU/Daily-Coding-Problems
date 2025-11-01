// Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
// lowest-frequency nodes (min-heap), then read each char's code from its
// root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
// Note: the README's example tree is illustrative, not a strict Huffman tree;
// this produces a correct, deterministic optimal-prefix Huffman mapping.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    long long freq;
    int order;       // deterministic tie-break by insertion order
    char ch;         // valid only for leaves
    Node *l, *r;
};

struct Cmp {
    bool operator()(const Node *a, const Node *b) const {
        if (a->freq != b->freq) return a->freq > b->freq;
        return a->order > b->order;
    }
};

void buildCodes(Node *n, string path, map<char, string> &codes) {
    if (!n->l && !n->r) {
        codes[n->ch] = path.empty() ? "0" : path; // single-node edge case
        return;
    }
    buildCodes(n->l, path + "0", codes);
    buildCodes(n->r, path + "1", codes);
}

int main() {
    // frequency dictionary
    vector<pair<char, long long>> freq = {
        {'c', 1}, {'a', 1}, {'t', 1}, {'s', 1}};

    priority_queue<Node *, vector<Node *>, Cmp> pq;
    int order = 0;
    for (auto &p : freq) pq.push(new Node{p.second, order++, p.first, nullptr, nullptr});

    while (pq.size() > 1) {
        Node *a = pq.top(); pq.pop();
        Node *b = pq.top(); pq.pop();
        pq.push(new Node{a->freq + b->freq, order++, 0, a, b});
    }
    Node *root = pq.top();

    map<char, string> codes;
    buildCodes(root, "", codes);

    for (auto &kv : codes) cout << kv.first << " -> " << kv.second << "\n";

    string word = "cats", encoded;
    for (char c : word) encoded += codes[c];
    cout << word << " -> " << encoded << "\n";
    return 0;
}
