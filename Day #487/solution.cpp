// Day 487: Find all cousins of a target node (same level, different parent).
// BFS level by level tracking each node's parent; on the target's level collect nodes
// whose parent differs from the target's parent. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> cousins(Node* root, int target) {
    if (!root) return {};
    queue<pair<Node*, Node*>> q; // node, parent
    q.push({root, nullptr});
    while (!q.empty()) {
        int sz = q.size();
        Node* targetParent = nullptr;
        vector<pair<Node*, Node*>> level;
        for (int i = 0; i < sz; i++) {
            auto [node, parent] = q.front(); q.pop();
            level.push_back({node, parent});
            if (node->val == target) targetParent = parent;
            if (node->left) q.push({node->left, node});
            if (node->right) q.push({node->right, node});
        }
        if (targetParent) {
            vector<int> res;
            for (auto [node, parent] : level)
                if (parent != targetParent && node->val != target)
                    res.push_back(node->val);
            return res;
        }
    }
    return {};
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(4);
    root->left->right = new Node(5);
    root->right->right = new Node(6);
    for (int v : cousins(root, 4)) cout << v << "\n"; // 6
    return 0;
}
