// Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
// same depth with different parent. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> cousins(Node* root, int target) {
    queue<pair<Node*, Node*>> q; // node, parent
    q.push({root, nullptr});
    while (!q.empty()) {
        int sz = q.size();
        Node* targetParent = nullptr;
        bool found = false;
        vector<pair<Node*, Node*>> level;
        for (int i = 0; i < sz; i++) {
            auto [node, par] = q.front(); q.pop();
            level.push_back({node, par});
            if (node->val == target) { targetParent = par; found = true; }
            if (node->left) q.push({node->left, node});
            if (node->right) q.push({node->right, node});
        }
        if (found) {
            vector<int> res;
            for (auto& [node, par] : level)
                if (par != targetParent) res.push_back(node->val);
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
    vector<int> res = cousins(root, 4);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) { if (i) cout << ", "; cout << res[i]; }
    cout << "]\n";
    return 0;
}
