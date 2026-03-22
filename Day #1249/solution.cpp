// Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    long val;
    Node* left;
    Node* right;
    Node(long v) : val(v), left(nullptr), right(nullptr) {}
};

bool isValid(Node* node, long lo, long hi) {
    if (!node) return true;
    if (node->val < lo || node->val > hi) return false;
    return isValid(node->left, lo, node->val) && isValid(node->right, node->val, hi);
}

bool isValidBST(Node* root) {
    return isValid(root, LONG_MIN, LONG_MAX);
}

int main() {
    // Valid tree: root 5, left 3 (2,5), right 8 (5,12)
    Node* root = new Node(5);
    root->left = new Node(3);
    root->left->left = new Node(2);
    root->left->right = new Node(5);
    root->right = new Node(8);
    root->right->left = new Node(5);
    root->right->right = new Node(12);
    cout << (isValidBST(root) ? "true" : "false") << endl;

    // Invalid tree: root 5, left 6 (6 > 5 violates)
    Node* bad = new Node(5);
    bad->left = new Node(6);
    bad->right = new Node(8);
    cout << (isValidBST(bad) ? "true" : "false") << endl;
    return 0;
}
