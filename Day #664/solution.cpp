// Day 664: Maximum path sum between any two nodes in a binary tree.
// Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *l, *r; Node(int v): val(v), l(nullptr), r(nullptr) {} };

int best;
int gain(Node* n) {
    if (!n) return 0;
    int lg = max(0, gain(n->l));
    int rg = max(0, gain(n->r));
    best = max(best, n->val + lg + rg);
    return n->val + max(lg, rg);
}
int maxPathSum(Node* root) { best = INT_MIN; gain(root); return best; }

int main() {
    // Tree:    -10
    //          /  \
    //         9    20
    //             /  \
    //            15   7   -> 15 + 20 + 7 = 42
    Node* root = new Node(-10);
    root->l = new Node(9);
    root->r = new Node(20);
    root->r->l = new Node(15);
    root->r->r = new Node(7);
    cout << maxPathSum(root) << "\n"; // 42
    return 0;
}
