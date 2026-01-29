// Root-to-leaf paths via DFS, appending to the current path and recording it at each leaf.
// Time O(n) nodes (O(n*h) including path copies), Space O(h) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

void dfs(Node* node, vector<int>& path, vector<vector<int>>& res) {
    if (!node) return;
    path.push_back(node->val);
    if (!node->left && !node->right) res.push_back(path);
    else { dfs(node->left, path, res); dfs(node->right, path, res); }
    path.pop_back();
}

vector<vector<int>> rootToLeafPaths(Node* root) {
    vector<vector<int>> res;
    vector<int> path;
    dfs(root, path, res);
    return res;
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->right->left = new Node(4);
    root->right->right = new Node(5);
    auto paths = rootToLeafPaths(root);
    cout << "[";
    for (size_t i = 0; i < paths.size(); i++) {
        cout << "[";
        for (size_t j = 0; j < paths[i].size(); j++)
            cout << paths[i][j] << (j + 1 < paths[i].size() ? ", " : "");
        cout << "]" << (i + 1 < paths.size() ? ", " : "");
    }
    cout << "]" << endl; // [[1, 2], [1, 3, 4], [1, 3, 5]]
    return 0;
}
