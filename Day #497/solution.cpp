// Convert binary tree to FULL binary tree by removing nodes with exactly one child.
// Post-order recursion: a one-child node is replaced by its processed child.
// Time: O(n), Space: O(h) recursion.
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* fullTree(Node* root) {
    if (!root) return nullptr;
    root->left = fullTree(root->left);
    root->right = fullTree(root->right);
    if (root->left && !root->right) return root->left;
    if (!root->left && root->right) return root->right;
    return root;
}

void preorder(Node* root, bool& first) {
    if (!root) return;
    if (!first) cout << ' ';
    cout << root->val;
    first = false;
    preorder(root->left, first);
    preorder(root->right, first);
}

int main() {
    Node* root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(2);
    root->left->left = new Node(3);
    root->left->left->right = new Node(5);
    root->right->right = new Node(4);
    root->right->right->left = new Node(6);
    root->right->right->right = new Node(7);

    root = fullTree(root);
    bool first = true;
    preorder(root, first);
    cout << '\n';
    return 0;
}
