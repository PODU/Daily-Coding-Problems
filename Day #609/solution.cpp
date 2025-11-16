// Day 609: Inorder successor in a BST using parent pointers.
// Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr, *parent = nullptr;
    Node(int v) : val(v) {}
};

Node* inorderSuccessor(Node* node) {
    if (!node) return nullptr;
    if (node->right) {
        Node* c = node->right;
        while (c->left) c = c->left;
        return c;
    }
    Node* cur = node;
    Node* p = node->parent;
    while (p && cur == p->right) { cur = p; p = p->parent; }
    return p;
}

Node* attach(Node* parent, Node* child) { if (child) child->parent = parent; return child; }

int main() {
    Node* n10 = new Node(10);
    Node* n5 = new Node(5);
    Node* n30 = new Node(30);
    Node* n22 = new Node(22);
    Node* n35 = new Node(35);
    n10->left = attach(n10, n5);
    n10->right = attach(n10, n30);
    n30->left = attach(n30, n22);
    n30->right = attach(n30, n35);

    Node* s = inorderSuccessor(n22);
    cout << (s ? to_string(s->val) : string("null")) << "\n"; // 30
    return 0;
}
