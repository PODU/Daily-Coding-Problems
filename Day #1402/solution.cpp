// Huffman coding: merge the two lowest-frequency nodes via a min-heap to build
// an optimal prefix tree, then DFS to assign codes (left=0, right=1).
// Time O(C log C) for C distinct chars, Space O(C).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int freq; char ch; Node *l, *r;
    Node(int f, char c) : freq(f), ch(c), l(nullptr), r(nullptr) {}
    Node(int f, Node* a, Node* b) : freq(f), ch(0), l(a), r(b) {}
};

void dfs(Node* n, string path, map<char,string>& codes) {
    if (!n->l && !n->r) { codes[n->ch] = path.empty() ? "0" : path; return; }
    dfs(n->l, path + "0", codes);
    dfs(n->r, path + "1", codes);
}

int main() {
    vector<pair<char,int>> freq = {{'c',1},{'a',4},{'t',3},{'s',2}};
    // min-heap on (freq, insertion order) for deterministic ties
    typedef tuple<int,int,Node*> Item;
    priority_queue<Item, vector<Item>, greater<Item>> pq;
    int cnt = 0;
    for (auto& p : freq) pq.push({p.second, cnt++, new Node(p.second, p.first)});
    while (pq.size() > 1) {
        Item it1 = pq.top(); pq.pop();
        Item it2 = pq.top(); pq.pop();
        int f1 = get<0>(it1), f2 = get<0>(it2);
        Node *n1 = get<2>(it1), *n2 = get<2>(it2);
        pq.push({f1 + f2, cnt++, new Node(f1 + f2, n1, n2)});
    }
    Node* root = get<2>(pq.top());
    map<char,string> codes;
    dfs(root, "", codes);
    for (auto& p : codes) cout << p.first << ": " << p.second << "\n";
    string word = "cats", enc;
    for (char c : word) enc += codes[c];
    cout << "encode(cats): " << enc << "\n";
    return 0;
}
