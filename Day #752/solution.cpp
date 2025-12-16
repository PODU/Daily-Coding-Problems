// Day 752: Level-order (BFS) traversal of a binary tree.
// Time: O(n), Space: O(w) where w is the max width of the tree.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> levelOrder(Node* root) {
    vector<int> out;
    if (!root) return out;
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        out.push_back(n->val);
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    return out;
}

int main() {
    //   1
    //  / \
    // 2   3
    //    / \
    //   4   5
    Node* root = new Node(1);
    root->left = new Node(2); root->right = new Node(3);
    root->right->left = new Node(4); root->right->right = new Node(5);

    vector<int> res = levelOrder(root);
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? ", " : "\n");
    return 0;
}
