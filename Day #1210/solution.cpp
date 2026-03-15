// Day 1210: Floor and ceiling of a target in a BST.
// Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v): val(v), l(nullptr), r(nullptr) {} };

struct Opt { bool has; int val; Opt(): has(false), val(0) {} };

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->val) root->l = insert(root->l, v);
    else root->r = insert(root->r, v);
    return root;
}

pair<Opt, Opt> floorCeil(Node* root, int x) {
    Opt fl, ce;
    while (root) {
        if (root->val == x) { fl.has = ce.has = true; fl.val = ce.val = x; return make_pair(fl, ce); }
        if (root->val < x) { fl.has = true; fl.val = root->val; root = root->r; }
        else { ce.has = true; ce.val = root->val; root = root->l; }
    }
    return make_pair(fl, ce);
}

int main() {
    Node* root = nullptr;
    int vals[] = {8, 4, 12, 2, 6, 10, 14};
    for (int v : vals) root = insert(root, v);
    pair<Opt, Opt> fc = floorCeil(root, 7);
    cout << "floor=" << (fc.first.has ? to_string(fc.first.val) : "None")
         << " ceiling=" << (fc.second.has ? to_string(fc.second.val) : "None") << "\n"; // floor=6 ceiling=8
    return 0;
}
