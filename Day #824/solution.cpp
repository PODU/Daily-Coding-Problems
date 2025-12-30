// Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
// Time: O(n), Space: O(h) recursion.
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

void preorder(Node* n, vector<int>& out) {
    if (!n) return;
    out.push_back(n->val);
    preorder(n->left, out);
    preorder(n->right, out);
}

int main() {
    // Tree1: 1 -> left 3 (left 5), right 2
    Node* t1 = new Node(1);
    t1->left = new Node(3);
    t1->left->left = new Node(5);
    t1->right = new Node(2);
    // Tree2: 2 -> left 1 (right 4), right 3 (right 7)
    Node* t2 = new Node(2);
    t2->left = new Node(1);
    t2->left->right = new Node(4);
    t2->right = new Node(3);
    t2->right->right = new Node(7);

    Node* m = merge(t1, t2);
    vector<int> out;
    preorder(m, out);
    cout << "[";
    for (size_t i = 0; i < out.size(); ++i) {
        cout << out[i];
        if (i + 1 < out.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
