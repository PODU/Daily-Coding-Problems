// Generate all distinct BSTs with values 1..N: recursively pick each value as
// root, combine all left/right subtree shapes. Count is Catalan(N).
// Time/Space O(Catalan(N) * N).
#include <iostream>
#include <vector>
#include <string>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}
};

std::vector<TreeNode*> build(int lo, int hi) {
    std::vector<TreeNode*> res;
    if (lo > hi) {
        res.push_back(nullptr);
        return res;
    }
    for (int root = lo; root <= hi; ++root) {
        std::vector<TreeNode*> lefts = build(lo, root - 1);
        std::vector<TreeNode*> rights = build(root + 1, hi);
        for (TreeNode* l : lefts) {
            for (TreeNode* r : rights) {
                TreeNode* node = new TreeNode(root);
                node->left = l;
                node->right = r;
                res.push_back(node);
            }
        }
    }
    return res;
}

void preorder(TreeNode* node, std::string& out) {
    if (!node) { out += "#"; return; }
    out += std::to_string(node->val);
    out += " ";
    preorder(node->left, out);
    out += " ";
    preorder(node->right, out);
}

int main() {
    int N = 3;
    std::vector<TreeNode*> trees = build(1, N);
    std::cout << trees.size() << std::endl; // 5 for N=3
    for (TreeNode* t : trees) {
        std::string s;
        preorder(t, s);
        std::cout << s << std::endl;
    }
    return 0;
}
