// Subtree check: at each node of s, test sameTree(node, t).
// Time: O(|s|*|t|) worst case. Space: O(height). (Optimal O(|s|+|t|) via
// tree serialization + KMP substring search; recursive version implemented.)
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v): val(v), left(nullptr), right(nullptr) {}
};

bool sameTree(Node* a, Node* b) {
    if (!a && !b) return true;
    if (!a || !b) return false;
    return a->val == b->val && sameTree(a->left, b->left) && sameTree(a->right, b->right);
}

bool isSubtree(Node* s, Node* t) {
    if (!s) return false;
    if (sameTree(s, t)) return true;
    return isSubtree(s->left, t) || isSubtree(s->right, t);
}

int main() {
    // s:        3
    //          / \
    //         4   5
    //        / \
    //       1   2
    Node* s = new Node(3);
    s->left = new Node(4);
    s->right = new Node(5);
    s->left->left = new Node(1);
    s->left->right = new Node(2);

    // t: 4 with children 1, 2
    Node* t = new Node(4);
    t->left = new Node(1);
    t->right = new Node(2);

    // t2: 4 with children 1, 0
    Node* t2 = new Node(4);
    t2->left = new Node(1);
    t2->right = new Node(0);

    cout << (isSubtree(s, t) ? "true" : "false") << "\n";
    cout << (isSubtree(s, t2) ? "true" : "false") << "\n";
    return 0;
}
