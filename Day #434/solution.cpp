// Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
// Single O(h) walk: node.val < n -> floor candidate (go right); node.val > n -> ceiling
// candidate (go left); equal -> both are n. O(h) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l = nullptr, *r = nullptr; Node(int v) : val(v) {} };

Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->val) root->l = insert(root->l, v);
    else root->r = insert(root->r, v);
    return root;
}

void floorCeil(Node* root, int n, bool& hf, int& f, bool& hc, int& c) {
    hf = hc = false;
    Node* cur = root;
    while (cur) {
        if (cur->val == n) { hf = hc = true; f = c = n; return; }
        else if (cur->val < n) { hf = true; f = cur->val; cur = cur->r; }
        else { hc = true; c = cur->val; cur = cur->l; }
    }
}

int main() {
    Node* root = nullptr;
    for (int v : {8, 4, 12, 2, 6, 10, 14}) root = insert(root, v);
    for (int n : {5, 8, 15, 1}) {
        bool hf, hc; int f, c;
        floorCeil(root, n, hf, f, hc, c);
        cout << "n=" << n << ": floor=" << (hf ? to_string(f) : "None")
             << ", ceiling=" << (hc ? to_string(c) : "None") << "\n";
    }
}
