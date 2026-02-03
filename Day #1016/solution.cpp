// Huffman coding: min-heap repeatedly merges two smallest nodes, then DFS assigns codes (left='0', right='1').
// Tie-break by insertion order for determinism. O(k log k) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int freq, order; char ch; Node *l, *r; };
struct Cmp {
    bool operator()(Node* a, Node* b) const {
        if (a->freq != b->freq) return a->freq > b->freq; // min-heap by freq
        return a->order > b->order;                       // then by insertion order
    }
};

void dfs(Node* n, string code, map<char,string>& codes) {
    if (!n->l && !n->r) { codes[n->ch] = code.empty() ? "0" : code; return; }
    dfs(n->l, code + "0", codes);
    dfs(n->r, code + "1", codes);
}

map<char,string> huffman(const map<char,int>& freqs) {
    priority_queue<Node*, vector<Node*>, Cmp> pq;
    int order = 0;
    for (auto& p : freqs) pq.push(new Node{p.second, order++, p.first, nullptr, nullptr});
    while (pq.size() > 1) {
        Node* a = pq.top(); pq.pop();
        Node* b = pq.top(); pq.pop();
        pq.push(new Node{a->freq + b->freq, order++, 0, a, b});
    }
    map<char,string> codes;
    dfs(pq.top(), "", codes);
    return codes;
}

int main() {
    map<char,int> freqs = {{'a',5},{'b',9},{'c',12},{'d',13},{'e',16},{'f',45}};
    auto codes = huffman(freqs);
    for (auto& p : codes) cout << p.first << ": " << p.second << "\n";
    return 0;
}
