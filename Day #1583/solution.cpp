// Day 1583: Bottom view of a binary tree.
// BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

vector<int> bottomView(Node* root) {
    map<int,int> hdVal;                 // hd -> latest (lowest) value
    if (!root) return {};
    queue<pair<Node*,int>> q;
    q.push({root, 0});
    while (!q.empty()) {
        auto [n, hd] = q.front(); q.pop();
        hdVal[hd] = n->val;             // overwrite -> deepest at this hd wins
        if (n->l) q.push({n->l, hd - 1});
        if (n->r) q.push({n->r, hd + 1});
    }
    vector<int> res;
    for (auto& [k, v] : hdVal) res.push_back(v);
    return res;
}

int main() {
    Node* root = new Node(5);
    root->l = new Node(3); root->r = new Node(7);
    root->l->l = new Node(1); root->l->r = new Node(4);
    root->r->l = new Node(6); root->r->r = new Node(9);
    root->l->l->l = new Node(0);
    root->r->r->l = new Node(8);
    auto r = bottomView(root);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [0, 1, 3, 6, 8, 9]
    return 0;
}
