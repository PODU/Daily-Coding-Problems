// Day 93: Largest BST subtree size. Post-order DFS returning (isBST, size, min,
// max) per node and combining children. O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(0), right(0) {} };

struct Info { bool bst; int size, mn, mx; };

int best = 0;
Info dfs(Node* n) {
    if (!n) return {true, 0, INT_MAX, INT_MIN};
    Info l = dfs(n->left), r = dfs(n->right);
    if (l.bst && r.bst && l.mx < n->val && n->val < r.mn) {
        int sz = l.size + r.size + 1;
        best = max(best, sz);
        return {true, sz, min(l.mn, n->val), max(r.mx, n->val)};
    }
    return {false, 0, 0, 0};
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5); root->left->left = new Node(1); root->left->right = new Node(8);
    root->right = new Node(15); root->right->right = new Node(7);
    dfs(root);
    cout << best << "\n"; // 3
    return 0;
}
