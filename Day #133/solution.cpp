// Day 133: Inorder successor in a BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right, *parent;
    Node(int v) : val(v), left(nullptr), right(nullptr), parent(nullptr) {}
};

Node* successor(Node* node) {
    if (!node) return nullptr;
    if (node->right) {
        Node* c = node->right;
        while (c->left) c = c->left;
        return c;
    }
    Node* p = node->parent;
    while (p && node == p->right) {
        node = p;
        p = p->parent;
    }
    return p;
}

Node* attach(Node* parent, Node* child) {
    if (child) child->parent = parent;
    return child;
}

int main() {
    Node* root = new Node(10);
    root->left = attach(root, new Node(5));
    root->right = attach(root, new Node(30));
    Node* n22 = new Node(22);
    Node* n35 = new Node(35);
    root->right->left = attach(root->right, n22);
    root->right->right = attach(root->right, n35);

    Node* s = successor(n22);
    cout << (s ? to_string(s->val) : "null") << endl; // 30
    return 0;
}
