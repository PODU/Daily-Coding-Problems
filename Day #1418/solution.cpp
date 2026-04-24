// Day 1418: inorder successor of a BST node using parent pointers.
// Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right, *parent;
    Node(int v) : val(v), left(nullptr), right(nullptr), parent(nullptr) {}
};

Node* successor(Node* node) {
    if (node->right) {
        Node* cur = node->right;
        while (cur->left) cur = cur->left;
        return cur;
    }
    Node* cur = node;
    while (cur->parent && cur == cur->parent->right) cur = cur->parent;
    return cur->parent;
}

Node* attach(Node* parent, Node* child) { if (child) child->parent = parent; return child; }

int main() {
    Node* root = new Node(10);
    root->left = attach(root, new Node(5));
    root->right = attach(root, new Node(30));
    Node* n22 = new Node(22);
    root->right->left = attach(root->right, n22);
    root->right->right = attach(root->right, new Node(35));

    Node* s = successor(n22);
    cout << (s ? to_string(s->val) : "null") << "\n"; // 30
    return 0;
}
