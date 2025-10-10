// Largest BST subtree: bottom-up DFS returning {isBST, size, min, max}; combine children.
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(nullptr),right(nullptr){} };

struct Info { bool isBST; int size; int mn; int mx; };

int best = 0;

Info dfs(Node* node) {
    if (!node) return {true, 0, INT_MAX, INT_MIN};
    Info l = dfs(node->left), r = dfs(node->right);
    if (l.isBST && r.isBST && node->val > l.mx && node->val < r.mn) {
        int sz = l.size + r.size + 1;
        best = max(best, sz);
        return {true, sz, min(node->val, l.mn), max(node->val, r.mx)};
    }
    return {false, 0, 0, 0};
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(15);
    root->left->left = new Node(1);
    root->left->right = new Node(8);
    root->right->right = new Node(7);
    dfs(root);
    cout << best << endl;
    return 0;
}
