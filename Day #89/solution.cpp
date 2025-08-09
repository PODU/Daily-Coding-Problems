// Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val; Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

bool valid(Node* n, long lo, long hi) {
    if (!n) return true;
    if (n->val < lo || n->val > hi) return false;
    return valid(n->left, lo, n->val) && valid(n->right, n->val, hi);
}

bool isBST(Node* root) { return valid(root, LONG_MIN, LONG_MAX); }

int main() {
    Node* a = new Node(5);
    a->left = new Node(3); a->right = new Node(8);
    a->left->left = new Node(2); a->left->right = new Node(4);
    cout << boolalpha << isBST(a) << "\n"; // true

    Node* b = new Node(5);
    b->left = new Node(3); b->right = new Node(8);
    b->left->right = new Node(6); // 6 > 5 in left subtree -> invalid
    cout << isBST(b) << "\n"; // false
    return 0;
}
