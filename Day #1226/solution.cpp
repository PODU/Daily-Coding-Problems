// Cousins in a binary tree: BFS level by level tracking parent; collect same-level
// nodes with a different parent than target. Time O(n), Space O(n).
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
        vector<pair<Node*, Node*>> level;
        Node* targetParent = nullptr;
        bool found = false;
        for (int i = 0; i < sz; i++) {
            auto cur = q.front(); q.pop();
            level.push_back(cur);
            if (cur.first->val == target) {
                found = true;
                targetParent = cur.second;
            }
            if (cur.first->left) q.push({cur.first->left, cur.first});
            if (cur.first->right) q.push({cur.first->right, cur.first});
        }
        if (found) {
            vector<int> res;
            for (auto& p : level)
                if (p.first->val != target && p.second != targetParent)
                    res.push_back(p.first->val);
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
    for (size_t i = 0; i < res.size(); i++) {
        if (i) cout << ", ";
        cout << res[i];
    }
    cout << "]\n";
    return 0;
}
