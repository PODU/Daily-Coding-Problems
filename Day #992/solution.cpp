// Day 992: Second largest node in a BST.
// The largest is the rightmost node; the 2nd largest is its left subtree's max,
// or its parent if it has no left child. O(h) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* insert(Node* root, int val) {
    if (!root) return new Node(val);
    if (val < root->val) root->left = insert(root->left, val);
    else root->right = insert(root->right, val);
    return root;
}

int secondLargest(Node* root) {
    Node *cur = root, *parent = nullptr;
    while (cur->right) { parent = cur; cur = cur->right; }
    if (cur->left) {
        cur = cur->left;
        while (cur->right) cur = cur->right;
        return cur->val;
    }
    return parent->val;
}

int main() {
    Node* root = nullptr;
    for (int v : {5, 3, 8, 2, 4, 7, 9}) root = insert(root, v);
    cout << secondLargest(root) << "\n"; // 8
    return 0;
}
