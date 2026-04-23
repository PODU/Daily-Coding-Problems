// Day 1411: Check if tree t is a subtree of tree s.
// Approach: recursive DFS — for each node of s try exact-match with t. O(|s|*|t|) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
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
    // s:      3            t:    4
    //        / \                / \
    //       4   5              1   2
    //      / \
    //     1   2
    Node* s = new Node(3);
    s->left = new Node(4);
    s->right = new Node(5);
    s->left->left = new Node(1);
    s->left->right = new Node(2);

    Node* t = new Node(4);
    t->left = new Node(1);
    t->right = new Node(2);

    cout << (isSubtree(s, t) ? "true" : "false") << "\n";
    return 0;
}
