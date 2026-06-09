// Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    long long val;
    Node *left, *right;
    Node(long long v) : val(v), left(nullptr), right(nullptr) {}
};

bool isValid(Node* n, long long lo, long long hi) {
    if (!n) return true;
    if (n->val < lo || n->val > hi) return false;
    return isValid(n->left, lo, n->val) && isValid(n->right, n->val, hi);
}

bool validate(Node* root) {
    return isValid(root, LLONG_MIN, LLONG_MAX);
}

int main() {
    // Valid BST: root 5, left 3 (2,4), right 8 (6,9)
    Node* root = new Node(5);
    root->left = new Node(3);
    root->left->left = new Node(2);
    root->left->right = new Node(4);
    root->right = new Node(8);
    root->right->left = new Node(6);
    root->right->right = new Node(9);

    // Invalid: root 5, left child 6
    Node* bad = new Node(5);
    bad->left = new Node(6);

    cout << (validate(root) ? "true" : "false") << "\n";
    cout << (validate(bad) ? "true" : "false") << "\n";
    return 0;
}
