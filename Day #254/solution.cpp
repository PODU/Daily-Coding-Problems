// Prune to full binary tree: post-order DFS; a node with exactly one child is
// replaced by that child. Returns new root. Time O(n), Space O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* prune(Node* root) {
    if (!root) return nullptr;
    root->left = prune(root->left);
    root->right = prune(root->right);
    if (root->left && !root->right) return root->left;
    if (!root->left && root->right) return root->right;
    return root;
}

void preorder(Node* n, vector<int>& out) {
    if (!n) return;
    out.push_back(n->val);
    preorder(n->left, out);
    preorder(n->right, out);
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

    root = prune(root);
    vector<int> pre;
    preorder(root, pre);
    cout << "Preorder after pruning:";
    for (int v : pre) cout << " " << v;
    cout << "\n";
    return 0;
}
