// Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
// Then print merged tree by level-order traversal (skipping null children).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* merge(Node* a, Node* b) {
    if (!a) return b;
    if (!b) return a;
    Node* n = new Node(a->val + b->val);
    n->left = merge(a->left, b->left);
    n->right = merge(a->right, b->right);
    return n;
}

int main() {
    Node* t1 = new Node(1);
    t1->left = new Node(3); t1->right = new Node(2);
    t1->left->left = new Node(5);

    Node* t2 = new Node(2);
    t2->left = new Node(1); t2->right = new Node(3);
    t2->left->right = new Node(4);
    t2->right->right = new Node(7);

    Node* m = merge(t1, t2);

    queue<Node*> q; q.push(m);
    vector<int> out;
    while (!q.empty()) {
        Node* c = q.front(); q.pop();
        out.push_back(c->val);
        if (c->left) q.push(c->left);
        if (c->right) q.push(c->right);
    }
    for (size_t i = 0; i < out.size(); i++) {
        cout << out[i];
        if (i + 1 < out.size()) cout << " ";
    }
    cout << "\n";
    return 0;
}
