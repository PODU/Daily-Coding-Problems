// Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
#include <bits/stdc++.h>
using namespace std;
struct Node { int v; Node *l, *r; Node(int x) : v(x), l(0), r(0) {} };
Node* insert(Node* root, int v) {
    if (!root) return new Node(v);
    if (v < root->v) root->l = insert(root->l, v); else root->r = insert(root->r, v);
    return root;
}
void floorCeil(Node* root, int x, Node*& fl, Node*& ce) {
    while (root) {
        if (root->v == x) { fl = ce = root; return; }
        if (root->v < x) { fl = root; root = root->r; }
        else { ce = root; root = root->l; }
    }
}
int main() {
    int vals[] = {8, 4, 12, 2, 6, 10, 14}; Node* root = 0;
    for (int v : vals) root = insert(root, v);
    Node *fl = 0, *ce = 0; floorCeil(root, 5, fl, ce);
    cout << "Floor: " << (fl ? to_string(fl->v) : "None")
         << ", Ceiling: " << (ce ? to_string(ce->v) : "None") << "\n"; // Floor: 4, Ceiling: 6
}
