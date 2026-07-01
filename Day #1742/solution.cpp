// Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
// Time/Space O(Catalan(N)*N).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<Node*> build(int lo, int hi) {
    vector<Node*> res;
    if (lo > hi) { res.push_back(nullptr); return res; }
    for (int r = lo; r <= hi; r++) {
        auto lefts = build(lo, r - 1);
        auto rights = build(r + 1, hi);
        for (auto l : lefts)
            for (auto ri : rights) {
                Node* root = new Node(r);
                root->left = l;
                root->right = ri;
                res.push_back(root);
            }
    }
    return res;
}

void preorder(Node* n, vector<int>& out) {
    if (!n) return;
    out.push_back(n->val);
    preorder(n->left, out);
    preorder(n->right, out);
}

int main() {
    int N = 3;
    auto trees = build(1, N);
    cout << trees.size() << "\n";
    for (auto t : trees) {
        vector<int> out;
        preorder(t, out);
        for (size_t i = 0; i < out.size(); i++)
            cout << out[i] << (i + 1 < out.size() ? "," : "");
        cout << "\n";
    }
    return 0;
}
