// Day 490: Bottom view of a binary tree.
// BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
// order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> bottomView(Node* root) {
    if (!root) return {};
    map<int, int> hdToVal; // hd -> value (last in BFS order = lowest)
    queue<pair<Node*, int>> q;
    q.push({root, 0});
    while (!q.empty()) {
        auto [node, hd] = q.front(); q.pop();
        hdToVal[hd] = node->val;
        if (node->left) q.push({node->left, hd - 1});
        if (node->right) q.push({node->right, hd + 1});
    }
    vector<int> res;
    for (auto& [hd, val] : hdToVal) res.push_back(val);
    return res;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(3);
    root->right = new Node(7);
    root->left->left = new Node(1);
    root->left->right = new Node(4);
    root->right->left = new Node(6);
    root->right->right = new Node(9);
    root->left->left->left = new Node(0);
    root->right->right->left = new Node(8);
    vector<int> res = bottomView(root);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]" << endl; // [0, 1, 3, 6, 8, 9]
    return 0;
}
