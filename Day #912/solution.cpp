// Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

vector<int> cousins(Node* root, int target) {
    if (!root) return {};
    queue<pair<Node*, Node*>> q; // {node, parent}
    q.push({root, nullptr});
    while (!q.empty()) {
        int sz = q.size();
        Node* targetParent = nullptr;
        vector<pair<Node*, Node*>> level;
        for (int i = 0; i < sz; ++i) {
            auto [node, par] = q.front(); q.pop();
            level.push_back({node, par});
            if (node->val == target) targetParent = par;
            if (node->left) q.push({node->left, node});
            if (node->right) q.push({node->right, node});
        }
        if (targetParent) { // target found on this level
            vector<int> res;
            for (auto& [node, par] : level)
                if (par != targetParent && node->val != target)
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

    auto print = [](const vector<int>& v) {
        cout << "[";
        for (size_t i = 0; i < v.size(); ++i) cout << v[i] << (i + 1 < v.size() ? ", " : "");
        cout << "]\n";
    };

    cout << "Cousins of 4: "; print(cousins(root, 4)); // [6]
    cout << "Cousins of 6: "; print(cousins(root, 6)); // [4, 5]
    return 0;
}
