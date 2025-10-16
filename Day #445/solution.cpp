// Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
// Postorder recursion, O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr, *right=nullptr; Node(int v):val(v){} };

Node* prune(Node* n) {
    if (!n) return nullptr;
    n->left = prune(n->left);
    n->right = prune(n->right);
    if (n->val == 0 && !n->left && !n->right) return nullptr;
    return n;
}

void show(Node* n, string prefix, string tag) {
    if (!n) return;
    cout << prefix << tag << n->val << "\n";
    show(n->left,  prefix + "  ", "L-- ");
    show(n->right, prefix + "  ", "R-- ");
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
    show(root, "", "");
    // 0
    //   L-- 1
    //   R-- 0
    //     L-- 1
    return 0;
}
