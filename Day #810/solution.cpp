// Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(0), right(0) {} };

vector<int> boustrophedon(Node* root) {
    vector<int> out;
    if (!root) return out;
    deque<Node*> q = {root};
    bool ltr = true;
    while (!q.empty()) {
        int sz = q.size();
        vector<int> level;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop_front();
            level.push_back(n->val);
            if (n->left) q.push_back(n->left);
            if (n->right) q.push_back(n->right);
        }
        if (!ltr) reverse(level.begin(), level.end());
        for (int v : level) out.push_back(v);
        ltr = !ltr;
    }
    return out;
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2); root->right = new Node(3);
    root->left->left = new Node(4); root->left->right = new Node(5);
    root->right->left = new Node(6); root->right->right = new Node(7);
    auto r = boustrophedon(root);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [1, 3, 2, 4, 5, 6, 7]
    return 0;
}
