// Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing the output order on alternate levels.
// Time: O(n), Space: O(n).
#include <iostream>
#include <vector>
#include <deque>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> boustrophedon(TreeNode* root) {
    vector<int> out;
    if (!root) return out;
    deque<TreeNode*> q;
    q.push_back(root);
    bool leftToRight = true;
    while (!q.empty()) {
        int sz = q.size();
        vector<int> level(sz);
        for (int i = 0; i < sz; i++) {
            TreeNode* node = q.front();
            q.pop_front();
            int idx = leftToRight ? i : sz - 1 - i;
            level[idx] = node->val;
            if (node->left) q.push_back(node->left);
            if (node->right) q.push_back(node->right);
        }
        for (int v : level) out.push_back(v);
        leftToRight = !leftToRight;
    }
    return out;
}

int main() {
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);
    root->left->left = new TreeNode(4);
    root->left->right = new TreeNode(5);
    root->right->left = new TreeNode(6);
    root->right->right = new TreeNode(7);
    vector<int> res = boustrophedon(root);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n"; // [1, 3, 2, 4, 5, 6, 7]
    return 0;
}
