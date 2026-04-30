// Day 1444: Convert a binary tree to a full binary tree by removing every node
// with exactly one child (its single child is promoted up).
// Post-order recursion. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* toFull(Node* root) {
    if (!root) return nullptr;
    root->left = toFull(root->left);
    root->right = toFull(root->right);
    if (root->left && !root->right) return root->left;   // one child -> promote
    if (!root->left && root->right) return root->right;
    return root;                                          // leaf or two children
}

void preorder(Node* r, vector<int>& out) {
    if (!r) return;
    out.push_back(r->val);
    preorder(r->left, out);
    preorder(r->right, out);
}

int main() {
    // Build the example tree.
    Node* n0 = new Node(0);
    n0->left = new Node(1); n0->right = new Node(2);
    n0->left->left = new Node(3);
    n0->left->left->right = new Node(5);
    n0->right->right = new Node(4);
    n0->right->right->left = new Node(6);
    n0->right->right->right = new Node(7);

    Node* full = toFull(n0);
    vector<int> out; preorder(full, out);
    cout << "Preorder of full tree:";
    for (int v : out) cout << " " << v;
    cout << "\n"; // 0 5 4 6 7
    return 0;
}
