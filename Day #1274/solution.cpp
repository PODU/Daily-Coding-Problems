// Day 1274: Prune a 0/1 binary tree, removing every subtree that contains only 0s.
// Post-order recursion: a node survives iff it is 1 or has a surviving child. O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

Node* prune(Node* node) {
    if (!node) return nullptr;
    node->left = prune(node->left);
    node->right = prune(node->right);
    if (node->val == 0 && !node->left && !node->right) return nullptr;
    return node;
}

string serialize(Node* node) {
    if (!node) return "null";
    return to_string(node->val) + "(" + serialize(node->left) + "," + serialize(node->right) + ")";
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
    // Pruned tree: 0(1(null,null),0(1(null,null),null))
    cout << serialize(root) << endl;
    return 0;
}
