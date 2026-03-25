// Day 1263: All root-to-leaf paths in a binary tree.
// DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

void dfs(Node* node, vector<int>& path, vector<vector<int>>& res) {
    if (!node) return;
    path.push_back(node->val);
    if (!node->left && !node->right) res.push_back(path);
    else { dfs(node->left, path, res); dfs(node->right, path, res); }
    path.pop_back();
}

vector<vector<int>> rootToLeaf(Node* root) {
    vector<vector<int>> res; vector<int> path;
    dfs(root, path, res);
    return res;
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->right->left = new Node(4);
    root->right->right = new Node(5);
    auto res = rootToLeaf(root);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "[";
        for (size_t j = 0; j < res[i].size(); ++j) { cout << res[i][j]; if (j + 1 < res[i].size()) cout << ", "; }
        cout << "]";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
