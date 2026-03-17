// DFS with BST pruning: skip left if val<a, skip right if val>b. O(n) worst.
// O(n) time worst, O(h) space (recursion).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int rangeSumBST(Node* node, int a, int b) {
    if (!node) return 0;
    if (node->val < a) return rangeSumBST(node->right, a, b);
    if (node->val > b) return rangeSumBST(node->left, a, b);
    return node->val + rangeSumBST(node->left, a, b) + rangeSumBST(node->right, a, b);
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(3);
    root->right = new Node(8);
    root->left->left = new Node(2);
    root->left->right = new Node(4);
    root->right->left = new Node(6);
    root->right->right = new Node(10);
    cout << rangeSumBST(root, 4, 9) << "\n";
    return 0;
}
