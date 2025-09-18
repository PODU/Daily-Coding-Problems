// Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
// Time O(n), Space O(log n) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l = nullptr, *r = nullptr; Node(int v):val(v){} };

Node* build(vector<int>& a, int lo, int hi) {
    if (lo > hi) return nullptr;
    int mid = (lo + hi) / 2;
    Node* n = new Node(a[mid]);
    n->l = build(a, lo, mid - 1);
    n->r = build(a, mid + 1, hi);
    return n;
}

int main() {
    vector<int> a = {-10, -3, 0, 5, 9};
    Node* root = build(a, 0, a.size() - 1);
    vector<int> order;
    queue<Node*> q; q.push(root);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        if (!n) continue;
        order.push_back(n->val);
        q.push(n->l); q.push(n->r);
    }
    cout << "[";
    for (size_t i = 0; i < order.size(); ++i) cout << order[i] << (i+1<order.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
