// Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
// per subtree, tracking the global best. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int v): val(v), left(0), right(0) {} };

struct Info { bool isBST; int size; long mn, mx; };

int best = 0;

Info dfs(TreeNode* node) {
    if (!node) return {true, 0, LONG_MAX, LONG_MIN};
    Info l = dfs(node->left), r = dfs(node->right);
    if (l.isBST && r.isBST && l.mx < node->val && node->val < r.mn) {
        int sz = l.size + r.size + 1;
        best = max(best, sz);
        return {true, sz, min((long)node->val, l.mn), max((long)node->val, r.mx)};
    }
    return {false, 0, LONG_MIN, LONG_MAX};
}

int largestBST(TreeNode* root) { best = 0; dfs(root); return best; }

int main() {
    TreeNode* root = new TreeNode(10);
    root->left = new TreeNode(5);
    root->right = new TreeNode(15);
    root->left->left = new TreeNode(1);
    root->left->right = new TreeNode(8);
    root->right->right = new TreeNode(7); // breaks BST at 15
    cout << largestBST(root) << "\n"; // 3 (subtree rooted at 5)
    return 0;
}
