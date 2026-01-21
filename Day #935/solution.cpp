// Day 935: Check if a binary tree is height-balanced.
// Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(0),right(0){} };

// Returns height if balanced, else -1.
int check(Node* n) {
    if (!n) return 0;
    int l = check(n->left);
    if (l == -1) return -1;
    int r = check(n->right);
    if (r == -1) return -1;
    if (abs(l - r) > 1) return -1;
    return 1 + max(l, r);
}
bool isBalanced(Node* root) { return check(root) != -1; }

int main() {
    // Balanced tree:        1
    //                      / \
    //                     2   3
    //                    /
    //                   4
    Node* a = new Node(1); a->left = new Node(2); a->right = new Node(3);
    a->left->left = new Node(4);
    cout << (isBalanced(a) ? "true" : "false") << "\n"; // true

    // Unbalanced (skewed):  1-2-3-4 chain on the left
    Node* b = new Node(1); b->left = new Node(2);
    b->left->left = new Node(3); b->left->left->left = new Node(4);
    cout << (isBalanced(b) ? "true" : "false") << "\n"; // false
    return 0;
}
