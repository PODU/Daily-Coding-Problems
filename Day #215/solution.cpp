// Day 215: Bottom view of a binary tree.
// Approach: BFS tracking horizontal distance; overwrite map[hd] so last (deepest) node wins. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> bottomView(Node* root) {
    map<int, int> hdMap; // hd -> node value (overwritten in BFS order)
    if (!root) return {};
    queue<pair<Node*, int>> q;
    q.push({root, 0});
    while (!q.empty()) {
        Node* node = q.front().first;
        int hd = q.front().second;
        q.pop();
        hdMap[hd] = node->val;
        if (node->left) q.push({node->left, hd - 1});
        if (node->right) q.push({node->right, hd + 1});
    }
    vector<int> res;
    for (auto& kv : hdMap) res.push_back(kv.second);
    return res;
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(3); root->right = new Node(7);
    root->left->left = new Node(1); root->left->right = new Node(4);
    root->right->left = new Node(6); root->right->right = new Node(9);
    root->left->left->left = new Node(0);
    root->right->right->left = new Node(8);
    auto r = bottomView(root);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
