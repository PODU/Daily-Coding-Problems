// Huffman coding: build tree via min-heap, merge two smallest nodes, assign 0/1 edges.
// Time: O(n log n) for n symbols. Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char ch; long long freq; int order; Node *l, *r;
    Node(char c, long long f, int o): ch(c), freq(f), order(o), l(nullptr), r(nullptr) {}
};

struct Cmp {
    bool operator()(Node* a, Node* b) const {
        if (a->freq != b->freq) return a->freq > b->freq;
        return a->order > b->order; // stable tie-break by insertion order
    }
};

void build(Node* n, string code, map<char,string>& out) {
    if (!n) return;
    if (!n->l && !n->r) { out[n->ch] = code.empty() ? "0" : code; return; }
    build(n->l, code + "0", out);
    build(n->r, code + "1", out);
}

int main() {
    vector<pair<char,long long>> freqs = {{'a',5},{'b',9},{'c',12},{'d',13},{'e',16},{'f',45}};
    priority_queue<Node*, vector<Node*>, Cmp> pq;
    int counter = 0;
    for (auto& p : freqs) pq.push(new Node(p.first, p.second, counter++));

    while (pq.size() > 1) {
        Node* a = pq.top(); pq.pop();
        Node* b = pq.top(); pq.pop();
        Node* m = new Node('\0', a->freq + b->freq, counter++);
        m->l = a; m->r = b;
        pq.push(m);
    }
    Node* root = pq.top();

    map<char,string> codes;
    build(root, "", codes);

    long long totalBits = 0;
    map<char,long long> fmap(freqs.begin(), freqs.end());
    for (auto& kv : codes) {
        cout << kv.first << ": " << kv.second << "\n";
        totalBits += (long long)kv.second.size() * fmap[kv.first];
    }
    cout << "Total encoded bits: " << totalBits << "\n";
    return 0;
}
