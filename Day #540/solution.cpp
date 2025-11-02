// Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> boustrophedon(Node* root) {
    vector<int> out;
    if (!root) return out;
    deque<Node*> q = {root};
    bool leftToRight = true;
    while (!q.empty()) {
        int sz = q.size();
        vector<int> level(sz);
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop_front();
            int idx = leftToRight ? i : sz - 1 - i;
            level[idx] = n->val;
            if (n->left) q.push_back(n->left);
            if (n->right) q.push_back(n->right);
        }
        for (int v : level) out.push_back(v);
        leftToRight = !leftToRight;
    }
    return out;
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2); root->right = new Node(3);
    root->left->left = new Node(4); root->left->right = new Node(5);
    root->right->left = new Node(6); root->right->right = new Node(7);

    vector<int> res = boustrophedon(root);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
