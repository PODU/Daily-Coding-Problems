// Validate BST with inclusive bounds: left<=node, right>=node (duplicates allowed both sides).
// Recursive (low,high) bound check. Time O(n), Space O(h). C++17.
#include <iostream>
#include <climits>
using namespace std;

struct TreeNode {
    long val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(long v): val(v), left(nullptr), right(nullptr) {}
};

bool isValid(TreeNode* node, long low, long high) {
    if (!node) return true;
    if (node->val < low || node->val > high) return false;
    return isValid(node->left, low, node->val) &&
           isValid(node->right, node->val, high);
}

bool isBST(TreeNode* root) {
    return isValid(root, LONG_MIN, LONG_MAX);
}

int main() {
    // Tree A (valid): root=5, left=3(l=2,r=5), right=8(l=8,r=9)
    TreeNode* a = new TreeNode(5);
    a->left = new TreeNode(3);
    a->right = new TreeNode(8);
    a->left->left = new TreeNode(2);
    a->left->right = new TreeNode(5);
    a->right->left = new TreeNode(8);
    a->right->right = new TreeNode(9);

    // Tree B (invalid): root=5, left=3, right=4
    TreeNode* b = new TreeNode(5);
    b->left = new TreeNode(3);
    b->right = new TreeNode(4);

    cout << (isBST(a) ? "true" : "false") << "\n";
    cout << (isBST(b) ? "true" : "false") << "\n";
    return 0;
}
