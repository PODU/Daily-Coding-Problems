// Day 1625: Inorder successor in BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is left child. O(h).
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
        Node* cur = node->right;
        while (cur->left) cur = cur->left;
        return cur;
    }
    Node* cur = node;
    while (cur->parent && cur->parent->right == cur) cur = cur->parent;
    return cur->parent;
}

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    Node* cur = root;
    while (true) {
        if (v < cur->val) {
            if (!cur->left) { cur->left = new Node(v); cur->left->parent = cur; return root; }
            cur = cur->left;
        } else {
            if (!cur->right) { cur->right = new Node(v); cur->right->parent = cur; return root; }
            cur = cur->right;
        }
    }
}

Node* find(Node* root, int v) {
    while (root && root->val != v) root = v < root->val ? root->left : root->right;
    return root;
}

int main() {
    Node* root = nullptr;
    for (int v : {10, 5, 30, 22, 35}) root = insert(root, v);
    Node* s = inorderSuccessor(find(root, 22));
    cout << "The inorder successor of 22 is " << (s ? to_string(s->val) : "None") << "." << endl;
    return 0;
}
