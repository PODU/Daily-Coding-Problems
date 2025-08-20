// Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

Node* prune(Node* root) {
    if (!root) return nullptr;
    root->left = prune(root->left);
    root->right = prune(root->right);
    if (root->val == 0 && !root->left && !root->right) return nullptr;
    return root;
}

void preorder(Node* r, string& out) {
    if (!r) return;
    out += to_string(r->val) + " ";
    preorder(r->left, out);
    preorder(r->right, out);
}

int main() {
    Node* root = new Node(0);
    root->left = new Node(1);
    root->right = new Node(0);
    root->right->left = new Node(1);
    root->right->right = new Node(0);
    root->right->left->left = new Node(0);
    root->right->left->right = new Node(0);
    root = prune(root);
    string out; preorder(root, out);
    cout << "preorder: " << out << endl; // 0 1 0 1
    return 0;
}
