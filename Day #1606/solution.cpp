// Day 1606: Check if a binary tree is height-balanced.
// Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int check(Node* root) { // returns height, or -1 if unbalanced
    if (!root) return 0;
    int l = check(root->left);
    if (l == -1) return -1;
    int r = check(root->right);
    if (r == -1) return -1;
    if (abs(l - r) > 1) return -1;
    return max(l, r) + 1;
}

bool isBalanced(Node* root) { return check(root) != -1; }

int main() {
    //      1
    //     / \
    //    2   3
    //   /
    //  4
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(4);
    cout << (isBalanced(root) ? "true" : "false") << "\n"; // true
    return 0;
}
