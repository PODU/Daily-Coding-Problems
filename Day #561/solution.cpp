// Sorted array -> height-balanced BST: recurse on mid=(lo+hi)/2 as root.
// Time: O(N), Space: O(N) for nodes + O(log N) recursion.
#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}
};

TreeNode* build(const vector<int>& a, int lo, int hi) {
    if (lo > hi) return nullptr;
    int mid = (lo + hi) / 2;
    TreeNode* root = new TreeNode(a[mid]);
    root->left = build(a, lo, mid - 1);
    root->right = build(a, mid + 1, hi);
    return root;
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6, 7};
    TreeNode* root = build(a, 0, (int)a.size() - 1);
    // Level-order (BFS) print.
    vector<int> out;
    queue<TreeNode*> q;
    if (root) q.push(root);
    while (!q.empty()) {
        TreeNode* n = q.front(); q.pop();
        out.push_back(n->val);
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    for (size_t i = 0; i < out.size(); ++i) {
        if (i) cout << ' ';
        cout << out[i];
    }
    cout << '\n';
    return 0;
}
