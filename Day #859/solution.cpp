// Day 859: Bottom view of a binary tree.
// Approach: BFS by horizontal distance; last node seen at each hd wins (lowest).
// Time: O(n log n) for ordering, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

vector<int> bottomView(Node* root) {
    map<int,int> hdToVal;
    queue<pair<Node*,int>> q;
    q.push({root, 0});
    while (!q.empty()) {
        auto [n, hd] = q.front(); q.pop();
        hdToVal[hd] = n->val;          // later (lower) overwrites
        if (n->l) q.push({n->l, hd - 1});
        if (n->r) q.push({n->r, hd + 1});
    }
    vector<int> res;
    for (auto& kv : hdToVal) res.push_back(kv.second);
    return res;
}

int main() {
    Node* root = new Node(5);
    root->l = new Node(3); root->r = new Node(7);
    root->l->l = new Node(1); root->l->r = new Node(4);
    root->r->l = new Node(6); root->r->r = new Node(9);
    root->l->l->l = new Node(0);
    root->r->r->l = new Node(8);

    auto res = bottomView(root);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) { if (i) cout << ", "; cout << res[i]; }
    cout << "]" << endl;
    return 0;
}
