// Day 135: Minimum root-to-leaf path sum (with path reconstruction).
// DFS over the tree. O(n) time, O(h) recursion space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

pair<int, vector<int>> minPath(Node* r) {
    if (!r) return {INT_MAX, {}};
    if (!r->left && !r->right) return {r->val, {r->val}};
    auto best = make_pair(INT_MAX, vector<int>());
    for (Node* c : {r->left, r->right}) {
        if (!c) continue;
        auto sub = minPath(c);
        if (sub.first < best.first) best = sub;
    }
    best.first += r->val;
    best.second.insert(best.second.begin(), r->val);
    return best;
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->left->right = new Node(2);
    root->right = new Node(5);
    root->right->right = new Node(1);
    root->right->right->left = new Node(-1);

    auto res = minPath(root);
    cout << res.first << " (path [";
    for (size_t i = 0; i < res.second.size(); i++)
        cout << res.second[i] << (i + 1 < res.second.size() ? ", " : "");
    cout << "])" << endl;
    return 0;
}
