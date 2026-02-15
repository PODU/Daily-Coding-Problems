// Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space
#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left, *right;
    TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}
};

int best;

int dfs(TreeNode* node) {
    if (!node) return 0;
    int l = max(0, dfs(node->left));
    int r = max(0, dfs(node->right));
    best = max(best, node->val + l + r);
    return node->val + max(l, r);
}

int maxPathSum(TreeNode* root) {
    best = INT_MIN;
    dfs(root);
    return best;
}

int main() {
    //       -10
    //       /  \
    //      9    20
    //          /  \
    //         15   7
    TreeNode* root1 = new TreeNode(-10);
    root1->left = new TreeNode(9);
    root1->right = new TreeNode(20);
    root1->right->left = new TreeNode(15);
    root1->right->right = new TreeNode(7);
    cout << "Max path sum: " << maxPathSum(root1) << "\n";

    //    1
    //   / \
    //  2   3
    TreeNode* root2 = new TreeNode(1);
    root2->left = new TreeNode(2);
    root2->right = new TreeNode(3);
    cout << "Max path sum: " << maxPathSum(root2) << "\n";
    return 0;
}
