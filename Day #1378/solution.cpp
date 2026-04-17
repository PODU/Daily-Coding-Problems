// Max path sum between any two nodes via DFS returning best downward gain.
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int v): val(v), left(nullptr), right(nullptr) {} };

int best;
int gain(TreeNode* n) {
    if (!n) return 0;
    int l = max(0, gain(n->left));
    int r = max(0, gain(n->right));
    best = max(best, n->val + l + r);
    return n->val + max(l, r);
}

int maxPathSum(TreeNode* root) {
    best = INT_MIN;
    gain(root);
    return best;
}

int main() {
    // Tree: -10 with children 9 and 20; 20 has children 15 and 7.
    TreeNode* root = new TreeNode(-10);
    root->left = new TreeNode(9);
    root->right = new TreeNode(20);
    root->right->left = new TreeNode(15);
    root->right->right = new TreeNode(7);
    cout << maxPathSum(root) << "\n";   // 42 (15 + 20 + 7)
    return 0;
}
