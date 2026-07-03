// Day 1755: Count nodes in a COMPLETE binary tree in better than O(n).
// Compare left/right spine heights: if equal, subtree is perfect (2^h - 1);
// else 1 + recurse on both children. Time O(log^2 n), Space O(log n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

int leftHeight(Node* n) {
    int h = 0;
    while (n) { h++; n = n->left; }
    return h;
}
int rightHeight(Node* n) {
    int h = 0;
    while (n) { h++; n = n->right; }
    return h;
}

int countNodes(Node* root) {
    if (!root) return 0;
    int lh = leftHeight(root), rh = rightHeight(root);
    if (lh == rh) return (1 << lh) - 1;          // perfect subtree
    return 1 + countNodes(root->left) + countNodes(root->right);
}

int main() {
    // Build a complete binary tree with 6 nodes (values 1..6):
    //          1
    //        /   \
    //       2     3
    //      / \   /
    //     4   5 6
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(4);
    root->left->right = new Node(5);
    root->right->left = new Node(6);

    cout << countNodes(root) << endl; // expected: 6
    return 0;
}
